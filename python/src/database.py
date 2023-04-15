from os import getenv
from dotenv import load_dotenv
from git import repo
import numpy as np
import pyodbc


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
        return value[0] if value else None

    def create_network(self, modelid: int):
        epoch = self.get_current_epoch(modelid)
        cursor = self.connection.cursor()
        cursor.execute(
            "INSERT INTO Network (ModelId, Epoch) OUTPUT inserted.NetworkId VALUES (?,?)",
            modelid,
            epoch,
        )

        value = cursor.fetchone()
        cursor.close()
        self.connection.commit()
        return value[0] if value else None

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
                values.append(item)

                if len(values) == MAX_INSERT_COUNT * 5:
                    print(len(values))
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
            values.append(item)
            if len(values) == MAX_INSERT_COUNT * 4:
                print(len(values))
                cursor.execute(max_insert_query, tuple(values))
                values = []

        count = len(values) // 4

        if count > 0:
            query = f'INSERT INTO Bias (NetworkId, Layer, X, Bias) VALUES {", ".join(count * ["(?,?,?,?)"])}'

            cursor.execute(query, tuple(values))

        cursor.close()
        self.connection.commit()




    def close(self):
        self.connection.close()
