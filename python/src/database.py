from os import getenv
import numpy as np
from dotenv import load_dotenv
from git import repo
import pyodbc


repo = repo.Repo(search_parent_directories=True)
git_commit = repo.head.object.hexsha

load_dotenv()


def create_database_connection():
    server = getenv("SQL_SERVER")
    port = getenv("SQL_PORT")
    database = getenv("SQL_DATABASE")
    username = getenv("SQL_USERNAME")
    password = getenv("SQL_PASSWORD")
    driver = "ODBC Driver 17 for SQL Server"
    return pyodbc.connect(
        f"DRIVER={driver};SERVER={server},{port};DATABASE={database};UID={username};PWD={password}"
    )

def ensure_connection(connection):
    if connection == None:
        return (create_database_connection(), True)
    else:
        return (connection, False)

def create_model(name: str, cube_type: str, connection=None):
    connection, is_new = ensure_connection(connection)

    cursor = connection.cursor()
    cursor.execute('INSERT INTO Model (ModelName, GitHash, CubeType) OUTPUT inserted.ModelId VALUES (?, ?, ?)', name, git_commit, cube_type)
    value = cursor.fetchone()
    connection.commit()

    if is_new:
        connection.close()

    return value[0]

def get_model_id(name: str, connection=None):
    connection, is_new = ensure_connection(connection)

    cursor = connection.cursor()
    cursor.execute('SELECT ModelId FROM Model WHERE ModelName = ?', name)
    row = cursor.fetchone()
    cursor.close()


    if is_new:
        connection.commit()
        connection.close()

    return row[0] if row else None

def get_nodes(model_id: int, connection=None):
    connection, is_new = ensure_connection(connection)

    cursor = connection.cursor()
    cursor.execute('SELECT Layer, NodeIndex, Weight, Bias FROM Node WHERE ModelId = ?', model_id)
    rows,  = cursor.fetchall()

    if is_new:
        connection.commit()
        connection.close()

    return rows

def insert_epoch(model_id: int, epoch: int, loss: np.float32, reward: np.float32, connection = None):
    connection, is_new = ensure_connection(connection)

    cursor = connection.cursor()
    cursor.execute('INSERT INTO Epoch (ModelId, Epoch, Loss, Reward) VALUES (?, ?, ?, ?)', model_id, epoch, loss, reward)
    connection.commit()

    if is_new:
        connection.close()

print(create_model('HI_WORLD_TESTING_PYTHON','Cube3x3'))
