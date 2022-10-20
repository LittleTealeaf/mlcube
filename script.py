from src import *
from git import Repo
from multiprocessing import Pool
import os
from tf_agents.replay_buffers import tf_uniform_replay_buffer

local_repo = Repo(path=".")
branch = local_repo.active_branch.name

random = Random()


def exponential_decay(initial, index, decay_rate, decay_interval=1):
    return initial * (decay_rate ** (index // decay_interval))


BATCH_SIZE = 512
REPLAY_BATCH_SIZE = 10_000
MAX_BUFFER_LENGTH = 1_000
TARGET_INTERVAL = 500
EVALUATE_INTERVAL = 10
SAVE_INTERVAL = 10


if __name__ == "__main__":
    with Pool(24) as pool:

        rewards = calculate_rewards(
            depth=7,base=10,
        )

        # CALCULATE REWARDS

        data_spec = (
            tf.TensorSpec([9 * 6 * 6], tf.float32, "state_1"),
            tf.TensorSpec([1], tf.int32, "choice"),
            tf.TensorSpec([9 * 6 * 6], tf.float32, "state_2"),
            tf.TensorSpec([1], tf.float32, "reward"),
        )

        replay_buffer = tf_uniform_replay_buffer.TFUniformReplayBuffer(
            data_spec, batch_size=BATCH_SIZE, max_length=MAX_BUFFER_LENGTH
        )

        agent = Agent([264, 202, 141, 80], f"agents/{branch}")

        while not os.path.exists("./stop"):

            epoch = agent.get_epoch()

            learning_rate = exponential_decay(
                exponential_decay(0.1, epoch, 0.99, TARGET_INTERVAL), epoch,0.99
            )
            epsilon = exponential_decay(0.5, epoch, 0.95, TARGET_INTERVAL)

            replay_buffer.add_batch(
                agent.create_replay_batch(
                    batch_size=BATCH_SIZE,
                    epsilon=epsilon,
                    scramble_depth=40,
                    random=random,
                    rewards=rewards,
                )
            )
            replay = replay_buffer.get_next(sample_batch_size=REPLAY_BATCH_SIZE)[0]
            training = agent.train_batch(replay, learning_rate=learning_rate, gamma=0.99)
            print(training)

            if epoch % TARGET_INTERVAL == 0:
                agent.update_target()

            if epoch % EVALUATE_INTERVAL == 0:
                print(agent.evaluate_network(
                    max_moves = 1_000,
                    scramble_depth=100,
                    rewards=rewards,
                    random=random
                ))

            if epoch % SAVE_INTERVAL == 0:
                agent.save()

        os.remove("./stop")

   