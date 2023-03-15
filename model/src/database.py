from os import getenv

import pymssql
from dotenv import load_dotenv

load_dotenv()


def create_database_connection() -> pymssql._pymssql.Connection:
    return pymssql.connect(
        host=getenv("SQL_HOST"),
        port=getenv("SQL_PORT"),
        user=getenv("SQL_USER"),
        password=getenv("SQL_PASSWORD"),
        database=getenv("SQL_DATABASE")
    )


def get_model_id(name: str, connection=None, create_missing=True) -> int:
    created_connection = not connection
    if created_connection:
        connection = create_database_connection()

    cursor = connection.cursor(as_dict=True)

    cursor.execute(f'SELECT ModelId FROM Models WHERE ModelName = \'{name}\'')

    row = cursor.fetchone()

    if row is None:
        if create_missing:
            cursor.execute(f'INSERT INTO Models (ModelName) OUTPUT Inserted.ModelId VALUES (\'{name}\')')
            row = cursor.fetchone()
        else:
            row = {'ModelId': -1}

    cursor.close()
    connection.commit()

    if created_connection:
        connection.close()

    return row['ModelId']
