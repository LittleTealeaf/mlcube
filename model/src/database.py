from dotenv import load_dotenv
from os import getenv
import pymssql

load_dotenv()


class DatabaseConnection:
    def __init__(self, rollback=False):
        self.connection = pymssql.connect(
            host=getenv("SQL_HOST"),
            port=getenv("SQL_PORT"),
            user=getenv("SQL_USER"),
            password=getenv("SQL_PASSWORD"),
            database=getenv("SQL_DATABASE"),
        )
        self.rollback = rollback
        self.closed = False

    def close(self):
      self.connection.close()
      self.closed = True

    def commit(self):
      if self.rollback:
        self.connection.rollback()
      else:
        self.connection.commit()


# connection = pymssql.connect(


# cursor = connection.cursor()
# cursor.execute('INSERT INTO test (id, value) OUTPUT INSERTED.id VALUES (1, 5), (2, 6)')
# row = cursor.fetchone()
# while row:
#   print("Inserted stuff: " + str(row[0]))
#   row = cursor.fetchone()
# connection.commit()
# connection.close()
