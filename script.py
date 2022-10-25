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


EVALUATE_INTERVAL = 10
SAVE_INTERVAL = 10
TARGET_INTERVAL = 250

BATCH_SIZE = 1024
REPLAY_BATCH_SIZE = 10_000
MAX_BUFFER_LENGTH = 1000


SCRAMBLE_DEPTH = 20
SCRAMBLE_DEPTH_MIN = 5
SCRAMBLE_DEPTH_MAX = 25
SCRAMBLE_WITH_RANGE = True

PREFILL_DATA = True
PREFILL_VALUE = None
PREFILL_EPSILON = 0
PREFILL_MAX = 100


if __name__ == "__main__":
    with Pool(24) as pool:

        rewards = calculate_rewards(
            depth=7,
            base=10,
        )

        data_spec = (
            tf.TensorSpec([9 * 6 * 6], tf.float32, "state_1"),
            tf.TensorSpec([1], tf.int32, "choice"),
            tf.TensorSpec([9 * 6 * 6], tf.float32, "state_2"),
            tf.TensorSpec([1], tf.float32, "reward"),
        )

        replay_buffer = tf_uniform_replay_buffer.TFUniformReplayBuffer(
            data_spec, batch_size=BATCH_SIZE, max_length=MAX_BUFFER_LENGTH
        )

        agent = Agent([700, 600, 500, 343, 200], f"agents/{branch}")

        # Pre-fill the replay data
        if PREFILL_DATA:
            prefill_iterations = min(
                REPLAY_BATCH_SIZE // BATCH_SIZE if not PREFILL_VALUE else PREFILL_VALUE,
                PREFILL_MAX,
            )

            for i in range(prefill_iterations):
                print(f"Prefilling Batch Data: {i+1}/{prefill_iterations}")
                replay_buffer.add_batch(
                    agent.create_replay_batch(
                        batch_size=BATCH_SIZE,
                        epsilon=PREFILL_EPSILON,
                        scramble_depth=random.randint(
                            SCRAMBLE_DEPTH_MIN, SCRAMBLE_DEPTH_MAX
                        )
                        if SCRAMBLE_WITH_RANGE
                        else SCRAMBLE_DEPTH,
                        random=random,
                        rewards=rewards,
                    )
                )

        while not os.path.exists("./stop"):

            epoch = agent.get_epoch()

            learning_rate = exponential_decay(
                exponential_decay(0.1, epoch, 0.99, TARGET_INTERVAL), epoch, 0.99
            )
            epsilon = exponential_decay(0.5, epoch, 0.95, TARGET_INTERVAL)

            replay_buffer.add_batch(
                agent.create_replay_batch(
                    batch_size=BATCH_SIZE,
                    epsilon=epsilon,
                    scramble_depth=random.randint(
                        SCRAMBLE_DEPTH_MIN, SCRAMBLE_DEPTH_MAX
                    )
                    if SCRAMBLE_WITH_RANGE
                    else SCRAMBLE_DEPTH,
                    random=random,
                    rewards=rewards,
                )
            )
            replay = replay_buffer.get_next(sample_batch_size=REPLAY_BATCH_SIZE)[0]
            training = agent.train_batch(
                replay, learning_rate=learning_rate, gamma=0.99
            )
            print(training)

            if epoch % TARGET_INTERVAL == 0:
                agent.update_target()

            if epoch % EVALUATE_INTERVAL == 0:
                print(
                    agent.evaluate_network(
                        max_moves=1_000,
                        scramble_depth=100,
                        rewards=rewards,
                        random=Random(branch),
                    )
                )

            if epoch % SAVE_INTERVAL == 0:
                agent.save()

        os.remove("./stop")
