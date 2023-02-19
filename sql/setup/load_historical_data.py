from dotenv import load_dotenv

load_dotenv()
import json
import pymssql

from os import getenv, path

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


def load_historical_data(name: str, epochs: list[Epoch]):
    cursor = connection.cursor(as_dict=True)

    cursor.execute(f'INSERT INTO Models (ModelName) OUTPUT Inserted.Modelid VALUES (\'{name}\')')
    row = cursor.fetchone()
    model_id = row['Modelid']

    max_len = 500

    segments = [
        epochs[i * max_len: i * max_len + max_len] for i in range((len(epochs) // max_len) + 1)
    ]

    for entries in segments:
        if len(entries) > 0:
            for entry in entries:
                entry.set_model_id(model_id)

            cursor.execute('INSERT INTO Epochs (Modelid, Epoch, Loss, Reward) VALUES ' + ",".join([
                f"({','.join([str(row.model_id), str(row.epoch), str(row.loss), str(row.reward)])})" for row in entries
            ]))
    cursor.close()
    connection.commit()


# %%

base_path = path.join('F:\\', 'Github', 'mlcube_archive', 'mlcube', 'agents')

# %%

for model_name in ['1', '2', '3', '4', '5', '6', '7', '8', '9', '10', '11', '12', '13', '15', '16', 'A-1', 'A-2', 'A-3',
                   'agent-1', 'agent-2', 'B-1', 'B-2', 'B-3', 'B-4', 'B-5', 'main-1', 'main-2', 'main-3', 'main-4',
                   'main-5', 'main-6', 'relu-1', 'relu-2', 'trial-1']:
    print(f'Loading {model_name}')
    # data = load_json()
    with open(path.join(base_path,model_name, 'epochs.json')) as f:
        data = json.load(f)
        print(f'Converting {model_name} to Epoch objects')
        epochs = [
            Epoch(row['epoch'], row['loss'], row['reward'])
            for row in data
        ]
        print(f"Pushing {model_name} to database" )
        load_historical_data(f'hist-{model_name}', epochs)
        del epochs
        del data

# %%

for model_name in ['sequential-2', 'sequential-3', 'sequential-4', 'sequential-5', 'sequential-6', 'sequential-7', 'sequential-8']:
    print(f'Loading {model_name}')
    # data = load_json()
    with open(path.join(base_path,model_name, 'epochs.json')) as f:
        data = json.load(f)
        print(f'Converting {model_name} to Epoch objects')
        epochs = [
            Epoch(row['epoch'], row['average_loss'], row['average_reward'])
            for row in data
        ]
        print(f"Pushing {model_name} to database" )
        load_historical_data(f'hist-{model_name}', epochs)
        del epochs
        del data