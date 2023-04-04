from os import getenv
from dotenv import load_dotenv
import pyodbc

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

def get_model_id(name: str, connection=None):
    connection, is_new = ensure_connection(connection)

    cursor = connection.cursor()
    cursor.execute('SELECT ModelId FROM Models WHERE ModelName = ?', name)
    row = cursor.fetchone()
    cursor.close()


    if is_new:
        connection.commit()
        connection.close()

    return row[0] if row else None


print(get_model_id("hist-5"))


# def get_model_id(name: str, connection=None, create_missing=False):
#     new_connection = not connection
#     if new_connection:
#         connection = create_database_connection()
#
#     cursor = connection.cursor()
#     cursor.execute(f'SELECT ModelId FROM Models WHERE ModelName = \'{name}\'')
#     row = cursor.fetchone()
#     cursor.close()
#
#     if new_connection:
#         connection.commit()
#         connection.close()
#
#     return row[0]
#
# connection = create_database_connection()
#
# model = get_model_id("hist-1",connection=connection)
# print(model)
