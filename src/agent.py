import json
import math
from multiprocessing import Pool, pool
import os
import tensorflow as tf
import numpy as np
from random import Random

from src.network import *
from src.environment import ACTIONS, create_environment, env_to_obs_tf, env_to_observations, COUNT_ACTIONS


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

    # def evaluate_network(self, max_moves=750, scramble_depth=100, rewards = {}, random = Random()):
    #     env = Environment()
    #     for _ in range(scramble_depth):
    #         env.apply_action(random.choice(ACTIONS))

    #     count = 0
    #     reward_max = env.reward(rewards)
    #     moves = []
    #     visited_states: list[int] = []

    #     while (not env.is_complete()) and count < max_moves and env.hash() not in visited_states:
    #         visited_states.append(env.hash())
    #         count = count + 1
    #         values = self.network.apply(
    #             tf.constant(env.to_observations(),dtype=tf.float32)
    #         )
    #         values_reshaped = tf.reshape(values,(18,))
    #         move_index = tf.argmax(values_reshaped).numpy()
    #         move = ACTIONS[move_index]
    #         env.apply_action(move)

    #         moves.append(move.name)
    #         # if move.name not in moves:
    #         #     moves[move.name] = 1
    #         # else:
    #         #     moves[move.name] = moves[move.name] + 1

    #         reward_max = max(reward_max,env.reward(rewards))


    #     solved = env.is_complete()
    #     reward_final = env.reward(rewards)

    #     result = {
    #         'epoch': self.get_epoch(),
    #         'solved': solved,
    #         'count': count,
    #         'moves': moves,
    #         'reward_max': reward_max,
    #         'reward_final': reward_final
    #     }

    #     self.evaluations.append(result)

    #     return result


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

    def create_replay_batch(self,batch_size=32,epsilon=0.5,scramble_depth=30,random=Random()):
        env = create_environment(scramble_depth=scramble_depth,random=random)

        state_1 = np.empty((batch_size,),dtype=tf.Tensor)
        choice_1 = np.zeros((batch_size,1),dtype=np.int32)
        state_2 = np.empty((batch_size,),dtype=tf.Tensor)

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

            if i < batch_size - 1:
                state_1[i+1] = state_2[i]

        return (
            tf.stack(state_1),
            tf.constant(choice_1,dtype=tf.int32),
            tf.stack(state_2)
        )
