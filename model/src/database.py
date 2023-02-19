from os import getenv

import pymssql
from dotenv import load_dotenv

load_dotenv()


def create_database_connection():
    return pymssql.connect(
        host=getenv("SQL_HOST"),
        port=getenv("SQL_PORT"),
        user=getenv("SQL_USER"),
        password=getenv("SQL_PASSWORD"),
        database=getenv("SQL_DATABASE")
    )
