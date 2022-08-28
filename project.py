import json
import os
import numpy as np
import tensorflow as tf
from random import Random
from keras.optimizers import SGD
from keras.activations import sigmoid

def create_cube():
    state = np.zeros(9 * 6 * 6, dtype=np.float32)
    for i in range(0,9*6*6,6):
        state[i + i // (9 * 6)] = 1
    return tf.constant(state,dtype=tf.float32,shape=(1,324))

def reward(state):
    return 1 if tf.math.reduce_all(tf.math.equal(state,create_cube())) else -1

class Move:
    def __init__(
        self, name: str, loops: list[list[int]], two: bool = False, prime: bool = False
    ):
        self.name = name
        copy = np.identity(9 * 6 * 6, dtype=np.float32)
        matrix = np.copy(copy)

        for offset in range(6):
            for loop in loops:
                for i in range(len(loop) - 1):
                    matrix[loop[i] + offset] = copy[loop[i+1] + offset]
                matrix[loop[-1] + offset] = copy[loop[0] + offset]

        if two:
            matrix = matrix @ matrix
        if prime:
            matrix = matrix.T

        self.tensor = tf.constant(matrix,dtype=tf.float32,name=f'MOVE{self.name}')

    def apply(self, state: np.ndarray[54, np.float32]):
        return tf.matmul(state,self.tensor)


def create_move(letter: str, loops: list[list[int]]) -> list[Move]:
    "Creates a move, it's prime syntax, and it's double syntax"
    return [
        Move(letter, loops),
        Move(f"{letter}P", loops, prime=True),
        Move(f"{letter}2", loops, two=True),
    ]


# Behold, python syntax
MOVES = [
    move
    for moves in [
        create_move(
            "R",
            [
                [20, 2, 42, 47],
                [23, 5, 39, 50],
                [26, 8, 36, 53],
                [27, 29, 35, 33],
                [28, 32, 34, 30],
            ],
        ),
        create_move(
            "U",
            [
                [20, 11, 38, 29],
                [19, 10, 37, 28],
                [18, 9, 36, 27],
                [8, 6, 0, 2],
                [7, 3, 1, 5],
            ],
        ),
        create_move(
            "L",
            [
                [18, 45, 44, 0],
                [21, 48, 41, 3],
                [24, 51, 38, 6],
                [11, 17, 15, 9],
                [14, 16, 12, 10],
            ],
        ),
        create_move(
            "D",
            [
                [24, 33, 42, 15],
                [25, 34, 43, 16],
                [26, 35, 44, 17],
                [45, 47, 53, 51],
                [46, 50, 52, 48],
            ],
        ),
        create_move(
            "F",
            [
                [6, 27, 47, 17],
                [7, 30, 46, 14],
                [8, 33, 45, 11],
                [18, 20, 26, 24],
                [19, 23, 25, 21],
            ],
        ),
        create_move(
            "B",
            [
                [36, 38, 44, 42],
                [37, 41, 43, 39],
                [29, 0, 15, 53],
                [32, 1, 12, 52],
                [35, 2, 9, 51],
            ],
        ),
    ]
    for move in moves
]

class Network:
    def __init__(self,layer_sizes: list[int], serialized = None, layers = None):
        self.layer_sizes = layer_sizes + [len(MOVES)]

        self.trainable_variables = []
        self.layers = layers or []

        if serialized:
            features = {}
            for i in range(len(layer_sizes) + 1):
                features[f'W{i}'] = tf.io.RaggedFeature(dtype=tf.string)
                features[f'b{i}'] = tf.io.RaggedFeature(dtype=tf.string)
            example = tf.io.parse_example(serialized,features)

            for i in range(len(layer_sizes) + 1):
                W = tf.variable(tf.io.parse_tensor(example[f'W{i}'][0],out_type=tf.float32, name=f'W{i}'))
                b = tf.Variable(tf.io.parse_tensor(example[f'b{i}'][0],out_type=tf.float32,name=f'b{i}'))
                self.layers.append((W,b))
        else:
            for i in range(len(self.layer_sizes)):
                length_prev = self.layer_sizes[i-1] if i > 0 else 54 * 6
                length_cur: int = self.layer_sizes[i]
                W = tf.Variable(
                    tf.random.normal([length_prev, length_cur],stddev=0.03),dtype=tf.float32
                )
                b = tf.Variable(tf.random.normal([length_cur],stddev=0.03),dtype=tf.float32)
                self.layers.append((W,b))

        for W,b in self.layers:
            self.trainable_variables.append(W)
            self.trainable_variables.append(b)

    def apply(self,input):
        x = input
        for W,b in self.layers:
            x = tf.matmul(x,W)
            x = tf.add(x, b)
            x = sigmoid(x)
        return x

    def copy(self):
        return Network(layers=self.layers)


    def serialize(self):
        features = {}
        for i in range(len(self.layers)):
            W, b = self.layers[i]
            features[f'W{i}'] = tf.train.Feature(bytes_list=tf.train.BytesList(value=[tf.io.serialize_tensor(W).numpy()]))
            features[f'b{i}'] = tf.train.Feature(bytes_list=tf.train.BytesList(value=[tf.io.serialize_tensor(b).numpy()]))

        return tf.train.Example(
            features = tf.train.Features(features)
        )

class Agent:
    def __init__(self,layer_sizes: list[int], dir: str = "agent"):
        self.network = None
        self.dir = dir



        if not self.network:
            self.network: Network = Network(layer_sizes)

        self.update_target()

    def update_target(self):
        self.target: Network = self.network.copy()
