import math
from os import getenv
from dotenv import load_dotenv
from git import repo
import numpy as np
import pyodbc
import sys


repo = repo.Repo(search_parent_directories=True)
git_commit = repo.head.object.hexsha

MAX_INSERT_COUNT = 400

load_dotenv()


class Database:
    def __init__(self):
        server = getenv("SQL_HOST")
        port = getenv("SQL_PORT")
        database = getenv("SQL_DATABASE")
        username = getenv("SQL_USER")
        password = getenv("SQL_PASSWORD")
        driver = "ODBC Driver 17 for SQL Server"

        self.connection = pyodbc.connect(
            f"DRIVER={driver};SERVER={server},{port};DATABASE={database};UID={username};PWD={password}"
        )

    def create_model(self, name: str, cube_type: str):
        cursor = self.connection.cursor()
        cursor.execute(
            "INSERT INTO Model (ModelName, GitHash, CubeType) OUTPUT inserted.ModelId VALUES (?,?,?)",
            name,
            git_commit,
            cube_type,
        )

        value = cursor.fetchone()
        cursor.close()
        self.connection.commit()
        return value[0] if value else None

    def get_model_id(self, name: str):
        cursor = self.connection.cursor()
        cursor.execute("SELECT ModelId FROM Model WHERE ModelName = ?", name)
        value = cursor.fetchone()
        cursor.close()
        return value[0] if value else None

    def get_current_epoch(self, modelid: int):
        cursor = self.connection.cursor()
        cursor.execute("get_current_epoch ?", modelid)
        value = cursor.fetchone()
        cursor.close()
        return value[0] if value else 0

    def create_network(self, modelid: int, is_target: bool):
        epoch = self.get_current_epoch(modelid)
        if epoch == None:
            epoch = 0
        cursor = self.connection.cursor()
        cursor.execute(
            "INSERT INTO Network (ModelId, Epoch, IsTarget) OUTPUT inserted.NetworkId VALUES (?,?,?)",
            modelid,
            epoch,
            is_target,
        )

        value = cursor.fetchone()
        cursor.close()
        self.connection.commit()
        return value[0] if value else None


    # TODO: FIX THE FACT THAT THESE INSERT INVALID FLOAT VALUES.. ALSO WE GOT AN ISSUE WITH THE NETWORK BLOATING TOO MUCH

    def insert_weights(self, networkid: int, layer: int, weights: list[list[float]]):
        cursor = self.connection.cursor()

        values = []

        max_insert_query = f'INSERT INTO Weight (NetworkId, Layer, X, Y, Weight) VALUES {", ".join(MAX_INSERT_COUNT * ["(?,?,?,?,?)"])}'

        for x, row in enumerate(weights):
            for y, item in enumerate(row):
                values.append(networkid)
                values.append(layer)
                values.append(x)
                values.append(y)
                values.append(float(item))

                if len(values) == MAX_INSERT_COUNT * 5:
                    cursor.execute(max_insert_query, tuple(values))
                    values = []

        count = len(values) // 5

        if count > 0:
            query = f'INSERT INTO Weight (NetworkId, Layer, X, Y, Weight) VALUES {", ".join(count * ["(?,?,?,?,?)"])}'

            cursor.execute(query, tuple(values))

        cursor.close()
        self.connection.commit()

    def insert_bias(self, networkid: int, layer: int, bias: list[float]):
        cursor = self.connection.cursor()

        values = []

        max_insert_query = f'INSERT INTO Bias (NetworkId, Layer, X, Bias) VALUES {", ".join(MAX_INSERT_COUNT * ["(?,?,?,?)"])}'

        for x, item in enumerate(bias):
            values.append(networkid)
            values.append(layer)
            values.append(x)
            values.append(float(item))
            if len(values) == MAX_INSERT_COUNT * 4:
                cursor.execute(max_insert_query, tuple(values))
                values = []

        count = len(values) // 4

        if count > 0:
            query = f'INSERT INTO Bias (NetworkId, Layer, X, Bias) VALUES {", ".join(count * ["(?,?,?,?)"])}'

            cursor.execute(query, tuple(values))

        cursor.close()
        self.connection.commit()

    def get_bias(self, networkid: int, layer: int):
        cursor = self.connection.cursor()
        cursor.execute(
            "SELECT Bias from Bias WHERE NetworkID = ? AND Layer = ? ORDER BY X",
            networkid,
            layer,
        )
        data = cursor.fetchall()
        cursor.close()
        return np.array([i[0] for i in data])

    def get_weights(self, networkid: int, layer: int, width: int):
        cursor = self.connection.cursor()
        cursor.execute(
            "SELECT Weight FROM Weight WHERE NetworkId = ? AND Layer = ? ORDER BY X, Y",
            networkid,
            layer,
        )
        data = cursor.fetchall()
        cursor.close()

        data = np.array([i[0] for i in data])
        data.resize((len(data) // width, width))
        return data

    def get_network_layer(self, networkid: int, layer: int):
        bias = self.get_bias(networkid, layer)
        weights = self.get_weights(networkid, layer, len(bias))
        return weights, bias

    def get_latest_network(self, modelid: int, is_target: bool = False):
        cursor = self.connection.cursor()

        cursor.execute(
            "SELECT TOP 1 NetworkId FROM Network WHERE ModelId = ? AND IsTarget = ? ORDER BY Epoch, NetworkId DESC",
            modelid,
            1 if is_target else 0,
        )
        data = cursor.fetchone()

        cursor.close()
        return data[0] if data else None

    def keep_latest_networks(self, modelid: int, count: int, is_target: bool = False):
        cursor = self.connection.cursor()
        cursor.execute(
            "SELECT NetworkId FROM Network WHERE ModelId = ? AND IsTarget = ? ORDER BY Epoch",
            modelid,
            1 if is_target else 0,
        )
        data = cursor.fetchall()
        data = [i[0] for i in data]
        cursor.close()
        delete_count = len(data) - count
        if delete_count > 0:
            for i in range(delete_count):
                cursor = self.connection.cursor()
                cursor.execute("delete_network ?", data[i])
                cursor.close()
            self.connection.commit()

    def insert_epoch(self, modelid: int, loss: float):
        epoch = self.get_current_epoch(modelid) + 1
        cursor = self.connection.cursor()
        if math.isinf(loss):
            loss = sys.float_info.max
        if math.isnan(loss):
            loss = 0

        cursor.execute(
            "INSERT INTO Epoch (ModelId, Epoch, Loss) VALUES (?,?,?)",
            modelid,
            epoch,
            loss
        )
        cursor.close()
        self.connection.commit()

    def upload_evaluation(
        self, modelid: int, seed: int, solved: bool, moves: list[tuple[str, float]]
    ):
        epoch = self.get_current_epoch(modelid)

        cursor = self.connection.cursor()
        cursor.execute(
            "INSERT INTO Evaluation (ModelId, Epoch, Solved, MoveCount, Seed) OUTPUT inserted.EvaluationId VALUES (?,?,?,?,?)",
            modelid,
            epoch,
            1 if solved else 0,
            len(moves),
            str(seed),
        )
        data = cursor.fetchone()
        cursor.close()
        id = data[0]

        cursor = self.connection.cursor()

        values = []

        max_insert_query = f'INSERT INTO EvaluationMove (EvaluationId, MoveIndex, MoveName, Reward) VALUES {", ".join(MAX_INSERT_COUNT * ["(?,?,?,?)"])}'

        for index, (name, reward) in enumerate(moves):
            values.append(id)
            values.append(index)
            values.append(name)
            values.append(reward)

            if len(values) == MAX_INSERT_COUNT * 4:
                cursor.execute(max_insert_query, tuple(values))

        count = len(values) // 4

        if count > 0:
            query = f'INSERT INTO EvaluationMove (EvaluationId, MoveIndex, MoveName, Reward) VALUES {", ".join(count * ["(?,?,?,?)"])}'

            cursor.execute(query, tuple(values))

        cursor.close()
        self.connection.commit()

    def close(self):
        self.connection.close()
