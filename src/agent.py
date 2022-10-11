from asyncio import constants
import json
import math
from multiprocessing import Pool, pool
import os
import tensorflow as tf
import numpy as np
from random import Random

from src.network import *
from src.environment import ACTION_COUNT, ACTIONS, ACTIONS_TENSOR, create_environment


class Agent:
    def __init__(self,layer_sizes: list[int], directory: str):
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

    def update_target(self):
        "Update the target network to match the current network"
        self.target = self.network.copy()

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


    @tf.function
    def run_cycle(
        self,
        env = create_environment(100),
        replay_length = tf.constant(10_000,dtype=tf.int64),
        gamma = tf.constant(0.7, dtype=tf.float64)
        ):

        loss = tf.constant(0,dtype=tf.float32)

        for _ in range(replay_length):
            output = self.network.apply(env)
            choice = tf.argmax(output,1)
            action_matrix = tf.gather_nd(ACTIONS_TENSOR,choice)
            env = tf.matmul(tf.reshape(env,[1,324]),action_matrix)
            tf.print(choice)
