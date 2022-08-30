import json
import os
import numpy as np
import tensorflow as tf
from random import Random
from keras.activations import sigmoid
from moves import MOVES

def create_cube():
    state = np.zeros(9 * 6 * 6, dtype=np.float32)
    for i in range(0,9*6*6,6):
        state[i + i // (9 * 6)] = 1
    return tf.constant(state,dtype=tf.float32,shape=(1,324))

def reward(state):
    return 1 if tf.math.reduce_all(tf.math.equal(state,create_cube())) else -1

class Network:
    def __init__(self,layer_sizes: list[int], serialized = None, layers = None):
        self.layer_sizes = layer_sizes + ([] if layers else [len(MOVES)])

        self.trainable_variables = []
        self.layers = []

        if layers:
            self.layers = layers
        elif serialized:
            features = {}
            for i in range(len(layer_sizes) + 1):
                features[f'W{i}'] = tf.io.RaggedFeature(dtype=tf.string)
                features[f'b{i}'] = tf.io.RaggedFeature(dtype=tf.string)
            example = tf.io.parse_example(serialized,features)

            for i in range(len(layer_sizes) - 1):
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

        print([f'{W.shape} {b.shape}' for W,b in self.layers])

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
        return Network(layer_sizes = self.layer_sizes, layers=self.layers)


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

    def create_replay(self,count,EPSILON = 0.5):
        random = Random()
        state_1_list = []
        for _ in range(count):
            cube = create_cube()
            # something's going on with this... it's coming out with FUNKY numbers
            for _ in range(1,100):
                cube = random.choice(MOVES).apply(cube)
            state_1_list.append(cube)
        state_1 = tf.constant(np.array(state_1_list))

        state_1_outputs = self.network.apply(state_1)
        state_1_choices = tf.argmax(state_1_outputs,0)
        state_1_choices = state_1_choices[0]
        state_1_choices = tf.map_fn(lambda i: i if random.random() > EPSILON else random.randint(0,len(MOVES)-1),state_1_choices)


        state_2_list = [MOVES[state_1_choices[i]].apply(state_1_list[i]) for i in range(count)]

        reward_1 = tf.constant(np.array([reward(state) for state in state_2_list]))

        state_2 = tf.constant(np.array(state_2_list))

        return state_1, state_1_choices, reward_1, state_2


agent = Agent([20,20])
replay = agent.create_replay(10)
print(replay)
