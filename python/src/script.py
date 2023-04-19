from agent import Agent
from mlcube import PyReplay2x2
from database import Database
import os


replay = PyReplay2x2(10_000)


UPDATE_TARGET_INTERVAL = 500
EVALUATION_INTERVAL = 50
SAVE_INTERVAL = 250
GAMMA = 0.9
TRAIN_SAMPLE_SIZE = 1000
EXPERIENCE_GATHER_SIZE = 2500


def calculate_epsilon(epoch):
    return (1 - ((epoch % UPDATE_TARGET_INTERVAL) / UPDATE_TARGET_INTERVAL)) * 0.5 + 0.25


def calculate_learning_rate(epoch):
    return 0.001 * (0.9 ** (epoch / UPDATE_TARGET_INTERVAL) ) * (0.99 ** ((epoch % UPDATE_TARGET_INTERVAL) / 100))


agent = Agent("Rust-Agent-Test-6", replay, [300,250,200,100], database=Database())

while not os.path.exists("./stop"):
    replay.reset()
    replay.scramble()
    epoch = agent.database.get_current_epoch(agent.model_id)

    EPSILON = calculate_epsilon(epoch)
    LEARNING_RATE = calculate_learning_rate(epoch)

    print("Epoch ", epoch)

    for _ in range(EXPERIENCE_GATHER_SIZE):
        agent.step_experience(EPSILON)

    agent.train(TRAIN_SAMPLE_SIZE, LEARNING_RATE, GAMMA)

    if epoch % UPDATE_TARGET_INTERVAL == 0:
        agent.update_target()

    if epoch % SAVE_INTERVAL == 0:
        agent.save()

    if epoch % EVALUATION_INTERVAL == 0:
        agent.evaluation()
