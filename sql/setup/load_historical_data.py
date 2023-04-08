# type: ignore
# %%
import json
from os import getenv, path
import os


import pymssql
from dotenv import load_dotenv

load_dotenv()

connection = pymssql.connect(
    host=getenv("SQL_HOST"),
    port=getenv("SQL_PORT"),
    user=getenv("SQL_USER"),
    password=getenv("SQL_PASSWORD"),
    database=getenv("SQL_DATABASE"),
)


class Epoch:
    def __init__(self, epoch: int, loss: float, reward: float):
        self.epoch = epoch
        self.loss = loss
        self.reward = reward
        self.model_id = None

    def set_model_id(self, model_id: int):
        self.model_id = model_id


def get_model_id(name: str):
    cursor = connection.cursor(as_dict=True)

    cursor.execute(f'SELECT ModelId FROM Model WHERE ModelName = \'{name}\'')

    row = cursor.fetchone()

    if row is None:
        cursor.execute(f'INSERT INTO Model (ModelName) OUTPUT Inserted.ModelId VALUES (\'{name}\')')
        row = cursor.fetchone()

    cursor.close()
    connection.commit()

    return row['ModelId']


def load_historical_data(name: str, epochs: list[Epoch]):
    model_id = get_model_id(name)

    cursor = connection.cursor(as_dict=True)
    cursor.execute(f'DELETE FROM Epoch WHERE ModelId = {model_id}')

    max_len = 500

    segments = [
        epochs[i * max_len: i * max_len + max_len] for i in range((len(epochs) // max_len) + 1)
    ]

    for entries in segments:
        if len(entries) > 0:
            for entry in entries:
                entry.set_model_id(model_id)

            cursor.execute('INSERT INTO Epoch (Modelid, Epoch, Loss, Reward) VALUES ' + ",".join([
                f"({','.join([str(row.model_id), str(row.epoch), str(row.loss), str(row.reward)])})" for row in entries
            ]))
    cursor.close()
    connection.commit()


# %%

base_path = path.split(path.split(os.getcwd())[0])[0] + "_archive"
base_path = path.join(base_path, 'mlcube')
base_path = path.join(base_path, 'agents')


# %% Simple Epochs

for model_name in ['1', '2', '3', '4', '5', '6', '7', '8', '9', '10', '11', '12', '13', '15', '16', 'A-1', 'A-2', 'A-3',
                   'agent-1', 'agent-2', 'B-1', 'B-2', 'B-3', 'B-4', 'B-5', 'main-1', 'main-2', 'main-3', 'main-4',
                   'main-5', 'main-6', 'relu-1', 'relu-2', 'trial-1']:
    print('Removing Prior Data')
    print(f'Loading {model_name}')
    # data = load_json()
    with open(path.join(base_path, model_name, 'epochs.json')) as f:
        data = json.load(f)
        print(f'Converting {model_name} to Epoch objects')
        epochs = [
            Epoch(row['epoch'], row['loss'], row['reward'])
            for row in data
        ]
        print(f"Pushing {model_name} to database")
        load_historical_data(f'hist-{model_name}', epochs)
        del epochs
        del data

# %% Sequential Epochs

for model_name in ['sequential-2', 'sequential-3', 'sequential-4', 'sequential-5', 'sequential-6', 'sequential-7',
                   'sequential-8']:
    print(f'Loading {model_name}')
    # data = load_json()
    with open(path.join(base_path, model_name, 'epochs.json')) as f:
        data = json.load(f)
        print(f'Converting {model_name} to Epoch objects')
        epochs = [
            Epoch(row['epoch'], row['average_loss'], row['average_reward'])
            for row in data
        ]
        print(f"Pushing {model_name} to database")
        load_historical_data(f'hist-{model_name}', epochs)
        del epochs
        del data


# %% Setup for Evaluations

class Evaluation:
    def __init__(self, object):
        self.epoch = object['epoch']
        self.solved = (1 if object['solved'] else 0) if 'solved' in object else 0
        self.move_count = object['count'] if 'count' in object else object['moves'] if 'moves' in object else 'NULL'
        self.max_reward = object['reward_max'] if 'reward_max' in object else 'NULL'
        self.final_reward = object['reward_final'] if 'reward_final' in object else 'NULL'
        self.moves = object['moves'] if 'moves' in object and type(object['moves']) == list else []


def load_historical_evaluations(model_name, evaluations):
    model_id = get_model_id(model_name)

    cursor = connection.cursor(as_dict=True)

    cursor.execute(f'''
    DELETE
FROM EvaluationMove
WHERE EvaluationId IN (SELECT EvaluationId
                       FROM Evaluation E
                       WHERE E.ModelId = {model_id})
    ''')
    cursor.execute(f'DELETE FROM Evaluation WHERE ModelId = {model_id}')

    max_len = 500

    for evaluation in evaluations:
        cursor.execute(f'''INSERT INTO Evaluation (ModelId, Epoch, Solved, MoveCount) OUTPUT Inserted.EvaluationId
        VALUES ({str(model_id)}, {str(evaluation.epoch)}, {str(evaluation.solved)}, {str(evaluation.move_count)})''')

        eval_id = cursor.fetchone()['EvaluationId']

        if len(evaluation.moves) > 0:
            for index, move in enumerate(evaluation.moves):
                cursor.execute(f'''INSERT INTO EvaluationMove (EvaluationId, MoveIndex, MoveName, Reward) VALUES
                ({str(eval_id)}, {str(index)}, '{str(move)}', 0)''')

    cursor.close()
    connection.commit()
    del cursor



# %% Importing data

agents = ['1', '2', '3', '4', '5', '6', '7', '8', '9', '10', '11', '12', '13', '15', '16', 'A-1', 'A-2', 'A-3',
          'agent-1', 'agent-2', 'B-1', 'B-2', 'B-3', 'B-4', 'B-5', 'main-1', 'main-2', 'main-3', 'main-4',
          'main-5', 'main-6', 'relu-1', 'relu-2', 'trial-1', 'sequential-2', 'sequential-3', 'sequential-4',
          'sequential-5', 'sequential-6', 'sequential-7',
          'sequential-8']

for model_name in agents:
    print(f'Loading {model_name}')
    # data = load_json()
    with open(path.join(base_path, model_name, 'evaluations.json')) as f:
        data = json.load(f)
        print(f'Converting {model_name} to Evaluation objects')
        evaluations = [Evaluation(value) for value in data]

        print(f'Pushing {model_name} to database')
        load_historical_evaluations(f'hist-{model_name}', evaluations)

        del data
        del evaluations

# %%

print("done")