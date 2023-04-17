from agent import Agent
from mlcube import PyReplay2x2
from database import Database
import os



replay = PyReplay2x2(10_000)


UPDATE_TARGET_INTERVAL = 500
EVALUATION_INTERVAL = 100
SAVE_INTERVAL = 50
GAMMA = 0.8
TRAIN_SAMPLE_SIZE = 1000

agent = Agent('Rust-Agent-Test-2', replay,[400,300,300], database=Database())

while not os.path.exists("./stop"):
    replay.reset()
    replay.scramble()
    epoch = agent.database.get_current_epoch(agent.model_id)

    print("Epoch ", epoch)

    for _ in range(1000):
        agent.step_experience(0.2)

    agent.train(TRAIN_SAMPLE_SIZE, 0.001 * 0.99 ** (epoch / UPDATE_TARGET_INTERVAL), GAMMA)

    if epoch % UPDATE_TARGET_INTERVAL == 0:
        agent.update_target()

    if epoch % SAVE_INTERVAL == 0:
        agent.save()

    if epoch % EVALUATION_INTERVAL == 0:
        agent.evaluation()
