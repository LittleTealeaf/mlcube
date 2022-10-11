import json
import math
from multiprocessing import Pool, pool
import os
import tensorflow as tf
import numpy as np
from random import Random

from src.network import *
from src.environment import ACTION_COUNT, Environment, ACTIONS, create_scrambled_environment


def pool_get_rewards(env,rewards: dict):
    hash = env.hash()
    return rewards.get(hash,0)


class Agent:
    def __init__(self, layer_sizes: list[int], directory: str):
        "Create a new agent"
        self.directory = directory
        self.network: Network = None
        self.target: Network = None
        self.layer_sizes = layer_sizes
        self.epochs = []
        self.evaluations = []

        if os.path.exists(directory):
            try:
                # Epoch History
                with open(os.path.join(directory, "epochs.json")) as file:
                    self.epochs = json.load(file)

                # Evaluation history
                with open(os.path.join(directory, "evaluations.json")) as file:
                    self.evaluations = json.load(file)

                # network and target
                network_data = tf.io.read_file(os.path.join(directory, "network"))
                target_data = tf.io.read_file(os.path.join(directory, "target"))

                self.network = Network(layer_sizes, serialized=network_data,output_size=ACTION_COUNT)
                self.target = Network(layer_sizes, serialized=target_data,output_size=ACTION_COUNT)

            except Exception as e:
                print(f"Tried opening and failed: {str(e)}")

        if not self.network:
            self.network = Network(layer_sizes,output_size=ACTION_COUNT)
            self.update_target()

    def evaluate_network(self, max_moves=750, scramble_depth=100, rewards = {}, random = Random()):
        env = Environment()
        for _ in range(scramble_depth):
            env.apply_action(random.choice(ACTIONS))

        count = 0
        reward_max = env.reward(rewards)
        moves = []
        visited_states: list[int] = []

        while (not env.is_complete()) and count < max_moves and env.hash() not in visited_states:
            visited_states.append(env.hash())
            count = count + 1
            values = self.network.apply(
                tf.constant(env.to_observations(),dtype=tf.float32)
            )
            values_reshaped = tf.reshape(values,(18,))
            move_index = tf.argmax(values_reshaped).numpy()
            move = ACTIONS[move_index]
            env.apply_action(move)

            moves.append(move.name)
            # if move.name not in moves:
            #     moves[move.name] = 1
            # else:
            #     moves[move.name] = moves[move.name] + 1

            reward_max = max(reward_max,env.reward(rewards))


        solved = env.is_complete()
        reward_final = env.reward(rewards)

        result = {
            'epoch': self.get_epoch(),
            'solved': solved,
            'count': count,
            'moves': moves,
            'reward_max': reward_max,
            'reward_final': reward_final
        }

        self.evaluations.append(result)

        return result


    def get_epoch(self):
        "Get the current epoch"
        return len(self.epochs)

    def save(self):
        "Save the network and target to disk"
        serialized_network = self.network.serialize()
        serialized_target = self.target.serialize()

        tf.io.write_file(os.path.join(self.directory, "network"), serialized_network)
        tf.io.write_file(os.path.join(self.directory, "target"), serialized_target)

        with open(os.path.join(self.directory, "epochs.json"), "w") as file:
            file.write(json.dumps(self.epochs))

        with open(os.path.join(self.directory, "evaluations.json"), "w") as file:
            file.write(json.dumps(self.evaluations))

    def update_target(self):
        "Update the target network to match the current network"
        self.target = self.network.copy()

    def run_cycle(
        self,
        pool: pool=None,
        replay_size=10_000,
        epsilon=0.2,
        moves_min=1,
        moves_max=40,
        learning_rate=0.1,
        gamma=0.5,
        rewards={},
        random=Random(),
    ):
        with tf.GradientTape() as tape:
            assert Pool is not None
            assert moves_max > moves_min
            assert len(rewards) > 0

            tape.watch(self.network.trainable_variables)

            moves_diff = moves_max - moves_min
            replay_scramble_depths = [
                i % moves_diff + moves_min for i in range(replay_size)
            ]
            ls_state_1 = pool.map(create_scrambled_environment, replay_scramble_depths)

            del replay_scramble_depths

            # Pushing state_1 through the network
            ls_observations_1 = pool.map(Environment.to_observations, ls_state_1)
            tf_state_1 = tf.constant(np.array(ls_observations_1), dtype=tf.float32)
            tf_output_1 = self.network.apply(tf_state_1)
            # TODO - correct this code
            tf_choices_1 = tf.argmax(tf_output_1, 2)

            del ls_observations_1

            # Scrambling actions
            np_choices_1 = tf_choices_1.numpy()
            for i in range(0,replay_size):
                if random.random() < epsilon:
                    np_choices_1[i] = [random.randint(0,ACTION_COUNT - 1)]

            tf_choices_1 = tf.constant(np_choices_1, dtype=tf.int32)

            # Filtering out to the output of the chosen actions
            tf_output_1_pruned = tf.gather(tf_output_1, tf_choices_1, batch_dims=2)

            # combining the ls_state_1 and np_choices_1
            ls_state_2 = pool.starmap(Environment.apply_action, [
                (ls_state_1[i], ACTIONS[np_choices_1[i][0]]) for i in range(replay_size)
            ])

            # Pushing state_2 through the target network
            ls_observations_2 = pool.map(Environment.to_observations, ls_state_2)
            tf_state_2 = tf.constant(np.array(ls_observations_2), dtype=tf.float32)
            tf_output_2 = self.target.apply(tf_state_2)

            del ls_observations_2

            # Get Rewards
            np_rewards = np.zeros((replay_size))
            for i in range(0,replay_size):
                np_rewards[i] = ls_state_2[i].reward(rewards)

            del ls_state_2

            # Get the max of the output of state_2
            tf_output_2_max = tf.reduce_max(tf_output_2, 2)

            tf_rewards = tf.reshape(tf.constant(np_rewards,dtype=tf.float32),[replay_size,1])

            tf_output_2_max_scaled = tf.multiply(tf_output_2_max,tf.constant(gamma,dtype=tf.float32))

            tf_target = tf.add(tf_output_2_max_scaled, tf_rewards)

            tf_loss = tf.square(tf.subtract(tf_target, tf_output_1_pruned))
            tf_loss_mean = tf.reduce_mean(tf_loss)

            tf_gradient = tape.gradient(tf_loss_mean,self.network.trainable_variables)

            optimizer = tf.keras.optimizers.SGD(learning_rate=learning_rate)

            optimizer.apply_gradients(zip(tf_gradient,self.network.trainable_variables))

            tf_rewards_mean = tf.reduce_mean(tf_rewards)

            output = {
                'epoch': self.get_epoch(),
                'loss': float(tf_loss_mean.numpy()),
                'reward': float(tf_rewards_mean.numpy()),
                'rate': float(learning_rate),
                'epsilon': float(epsilon),
                'gamma': float(gamma),
            }

            self.epochs.append(output)

            return output











    # def run_cycle(self,replay_size=1000,epsilon=0.2,moves_min=1,moves_max=40, learning_rate=0.1, gamma = 0.5):
    #   "Run a cycle of training and evaluation"

    #   # This is what takes up most of the time
    #   replay = self.create_replay_deprecated(replay_size=replay_size,epsilon=epsilon,moves_min=moves_min,moves_max=moves_max)

    #   loss, gradient, t_rewards = self.train_replay_deprecated(replay,gamma=gamma)

    #   avg_reward = float(tf.math.reduce_mean(t_rewards).numpy())

    #   loss_avg = float(tf.math.reduce_mean(loss).numpy())

    #   optimizer = SGD(learning_rate=learning_rate)
    #   optimizer.apply_gradients(zip(gradient,self.network.trainable_variables))

    #   self.epochs.append({
    #     'epoch': self.get_epoch(),
    #     'loss': loss_avg,
    #     'reward': avg_reward
    #   })

    #   return loss_avg, loss_avg**0.5,avg_reward


# https://czxttkl.com/2015/09/28/python-multiprocessing-map-function-with-shared-memory-object-as-additional-parameter/


# replay_environments = pool.map(create_scrambled_env,replay_scramble_depths)
# tf_state_1 = tf.constant(np.array(replay_environments),dtype=tf.float32)
# tf_state_1_output = self.network.apply(tf_state_1)
# tf_state_1_choice = tf.argmax(tf_state_1_output,2)


# def create_replay_deprecated(self,replay_size=1_000,epsilon=0.2,moves_min=1,moves_max=40, pool = None):
#   "Create a replay of random moves"

#   state_1 = np.zeros(shape=(replay_size,1,9*6*6),dtype=np.float32)
#   choice = np.zeros(shape=(replay_size,1,1),dtype=np.float32)
#   state_2 = np.zeros(shape=(replay_size,1,9*6*6),dtype=np.float32)
#   rewards = np.zeros(shape=(replay_size,1,1),dtype=np.float32)

#   random = Random()
#   entries_per_move = int(replay_size / (moves_max - moves_min)) + 1
#   cubes = [Environment() for _ in range(entries_per_move)]
#   for _ in range(moves_min):
#     for cube in cubes:
#       cube.apply_action(random.choice(ACTIONS))
#   for i in range(replay_size):
#     cube = cubes[i%len(cubes)]
#     state_1[i] = cube.to_observations_deprecated()
#     value = self.network.apply(tf.constant(state_1[i],dtype=tf.float32))
#     choice[i] = tf.argmax(value,axis=1).numpy() if random.random() >= epsilon else [random.randint(0,17)]
#     cube.apply_action(ACTIONS[int(choice[i][0])])
#     state_2[i] = cube.to_observations_deprecated()
#     cube.apply_action(random.choice(ACTIONS))
#     rewards[i] = cube.reward()


#   return (state_1,choice,state_2,rewards)
