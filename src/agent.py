import json
import math
from multiprocessing import Pool, pool
import os
import tensorflow as tf
import numpy as np
from random import Random

from keras import Sequential, Model
from keras.layers import Dense
from keras.activations import relu
from keras.initializers.initializers_v2 import VarianceScaling

from src.network import *
from src.environment import ACTION_COUNT, REWARDS, Environment, ACTIONS, create_scrambled_environment


def pool_get_rewards(env,rewards: dict):
    hash = env.hash()
    return rewards.get(hash,0)

def create_dense_layer(layer_size):
    return Dense(
        layer_size,activation= relu,kernel_initializer=VarianceScaling(
            scale=2.0,
            mode='fan_in',
            distribution='truncated_normal'
        )
    )

class Agent:
    def __init__(self,layer_sizes: list[int], directory: str):
        "Create a new agent, or load if it exists in the directory"
        self.directory: str = directory
        self.network: Model = None
        self.target: Model = None
        self.layer_sizes = layer_sizes

        self.evaluations = []
        self.log = []


        # load network and target

        if not self.network:
            self.network = Sequential(
                [create_dense_layer(size) for size in layer_sizes] + [
                    Dense(
                        ACTION_COUNT,
                        activation=None,
                        kernel_initializer=tf.keras.initializers.RandomUniform(
                            minval=-0.03, maxval=0.03),
                        bias_initializer=tf.keras.initializers.Constant(-0.2)
                    )
                ]
            )


class Agent_:
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

                self.network = Network(layer_sizes, serialized=network_data)
                self.target = Network(layer_sizes, serialized=target_data)

            except Exception as e:
                print(f"Tried opening and failed: {str(e)}")

        if not self.network:
            self.network = Network(layer_sizes)
            self.update_target()

    def evaluate_network(self, max_moves=750, scramble_depth=100, rewards = {}, random = Random()):
        env = Environment()
        for _ in range(scramble_depth):
            env.apply_action(random.choice(ACTIONS))

        count = 0
        reward_max = env.reward(rewards)
        moves = {}
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

            if move.name not in moves:
                moves[move.name] = 1
            else:
                moves[move.name] = moves[move.name] + 1

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
            ls_rewards = pool.starmap(pool_get_rewards,[
                (i,rewards) for i in ls_state_2
            ])

            del ls_state_2

            # Get the max of the output of state_2
            tf_output_2_max = tf.reduce_max(tf_output_2, 2)

            tf_rewards = tf.reshape(tf.constant(np.array(ls_rewards),dtype=tf.float32),[replay_size,1])

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
