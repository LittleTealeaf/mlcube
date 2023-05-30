from agent import Agent
from mlcube import PyReplay2x2
from database import Database
import os


replay = PyReplay2x2(100_000)
EXPERIENCE_GATHER_SIZE = 1_000
TRAIN_SAMPLE_SIZE = 500

UPDATE_TARGET_INTERVAL = 500
EVALUATION_INTERVAL = 50

SAVE_INTERVAL = 250
PURGE_INTERVAL = 1000
KEEP_COUNT = 2


def calculate_gamma(epoch):
    return min(0.05 * (epoch // UPDATE_TARGET_INTERVAL), 1.0)

def calculate_epsilon(epoch):
    return (1 - ((epoch % UPDATE_TARGET_INTERVAL) / UPDATE_TARGET_INTERVAL)) * 0.7
# Keep constant or decrease every target interval


def calculate_learning_rate(epoch):
    return 0.01 * (0.95 ** (epoch / UPDATE_TARGET_INTERVAL) ) * (0.99 ** (epoch % UPDATE_TARGET_INTERVAL))

agent = Agent("Rust-Agent-11-1", replay, [300,300,300,200,200], database=Database())

while not os.path.exists("./stop"):
    replay.reset()
    replay.scramble()
    epoch = agent.database.get_current_epoch(agent.model_id)

    EPSILON = calculate_epsilon(epoch)
    LEARNING_RATE = calculate_learning_rate(epoch)
    GAMMA = calculate_gamma(epoch)

    print("Epoch ", epoch)

    for _ in range(EXPERIENCE_GATHER_SIZE):
        agent.step_experience(EPSILON)

        if replay.is_solved():
            replay.scramble()


    agent.train(TRAIN_SAMPLE_SIZE, LEARNING_RATE, GAMMA)

    if epoch % UPDATE_TARGET_INTERVAL == 0:
        agent.update_target()

    if epoch % SAVE_INTERVAL == 0:
        agent.save()

    if epoch % PURGE_INTERVAL == 0:
        agent.purge_networks(keep_count=KEEP_COUNT)

    if epoch % EVALUATION_INTERVAL == 0:
        agent.evaluation()

agent.save()
agent.purge_networks(keep_count=1)

os.remove("./stop")
