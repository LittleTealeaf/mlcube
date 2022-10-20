import json
import math
from multiprocessing import Pool, pool
import os
import tensorflow as tf
import numpy as np
from random import Random

from src.network import *
from src.environment import ACTIONS, create_environment, env_is_complete, env_to_obs_tf, env_to_observations, COUNT_ACTIONS, get_reward, hash_env


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

                self.network = Network(layer_sizes, serialized=network_data,output_size=COUNT_ACTIONS)
                self.target = Network(layer_sizes, serialized=target_data,output_size=COUNT_ACTIONS)

            except Exception as e:
                print(f"Tried opening and failed: {str(e)}")

        if not self.network:
            self.network = Network(layer_sizes,output_size=COUNT_ACTIONS)
            self.update_target()

    def evaluate_network(self,max_moves=1_000,scramble_depth=100,rewards={},random=Random()):
        env = create_environment(scramble_depth=scramble_depth,random=random)

        count = 0
        reward_max = get_reward(env,rewards=rewards)
        moves = []
        visited_states = []

        while not env_is_complete(env) and count < max_moves and hash_env(env) not in visited_states:
            visited_states.append(hash_env(env))
            count = count + 1
            values = self.network.apply(env_to_obs_tf(env))
            values_reshaped = tf.reshape(values,(18,))
            move_index = tf.argmax(values_reshaped).numpy()
            move = ACTIONS[move_index]
            env = move.apply(env)

            moves.append(move.name)

            reward_max = max(reward_max,get_reward(env))

        solved = env_is_complete(env)
        reward_final = get_reward(env)

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

    def create_replay_batch(self,batch_size=32,epsilon=0.5,scramble_depth=30,random=Random(),rewards={}):
        env = create_environment(scramble_depth=scramble_depth,random=random)

        state_1 = np.empty((batch_size,),dtype=tf.Tensor)
        choice_1 = np.zeros((batch_size,1),dtype=np.int32)
        state_2 = np.empty((batch_size,),dtype=tf.Tensor)
        reward_2 = np.zeros((batch_size,1),dtype=np.float32)

        for i in range(batch_size):
            if i == 0:
                state_1[i] = tf.constant(env_to_obs_tf(env),dtype=tf.float32)

            if random.random() > epsilon:
                output = self.network.apply(state_1[i])
                choice_1[i][0] = tf.argmax(output,axis=1).numpy()[0]
            else:
                choice_1[i][0] = random.randint(0,17)

            env = ACTIONS[choice_1[i][0]].apply(env)
            state_2[i] = env_to_obs_tf(env)
            env_hash = hash_env(env)

            reward_2[i][0] = rewards[env_hash] if env_hash in rewards else 0

            if i < batch_size - 1:
                state_1[i+1] = state_2[i]

        return (
            tf.stack(state_1),
            tf.constant(choice_1,dtype=tf.int32),
            tf.stack(state_2),
            tf.constant(reward_2,dtype=tf.float32)
        )

    def train_batch(self,batch,gamma=0.99,learning_rate=0.1):
        state_1,choice_1,state_2,reward_2 = batch

        with tf.GradientTape() as tape:

            tape.watch(self.network.trainable_variables)

            output_1 = self.network.apply(state_1)
            output_1_gathered = tf.gather(output_1,choice_1,batch_dims=1)
            output_2 = self.target.apply(state_2)
            output_2_gathered = tf.reduce_max(output_2,axis=1)

            output_2_gathered_scaled = tf.multiply(output_2_gathered,gamma)

            loss_raw = tf.reshape(output_2_gathered_scaled,(output_2_gathered_scaled.shape[0],1)) - output_1_gathered - reward_2

            loss = tf.math.square(loss_raw)

            loss_mean = tf.reduce_mean(loss)

            gradient = tape.gradient(loss_mean,self.network.trainable_variables)

            optimizer = tf.keras.optimizers.SGD(learning_rate=learning_rate)

            optimizer.apply_gradients(zip(gradient,self.network.trainable_variables))

            values = {
                'epoch': self.get_epoch(),
                'average_loss': float(loss_mean.numpy()),
                'average_reward': float(tf.reduce_mean(tf.reshape(reward_2,(reward_2.shape[0],))))
            }

            self.epochs.append(values)

            return values
