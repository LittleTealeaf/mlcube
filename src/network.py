import tensorflow as tf
from keras.activations import sigmoid
import numpy as np

LAYER_SIZE_INPUT = 9 * 6 * 6
LAYER_SIZE_OUTPUT = 18


class Network:
    def __init__(self, layer_sizes: list[int], serialized=None, layers=None):
        self.layer_sizes = layer_sizes + [LAYER_SIZE_OUTPUT]
        self.trainable_variables = []
        self.layers = []

        if layers:
            self.layers = layers
        elif serialized:
            features = {}
            for i in range(len(layer_sizes) + 1):
                features[f"W{i}"] = tf.io.RaggedFeature(dtype=tf.string)
                features[f"b{i}"] = tf.io.RaggedFeature(dtype=tf.string)
            example = tf.io.parse_example(serialized, features)
            for i in range(len(layer_sizes) + 1):

                W_tensor = tf.io.parse_tensor(
                    example[f"W{i}"][0], out_type=tf.float32, name=f"W{i}"
                )
                b_tensor = tf.io.parse_tensor(
                    example[f"b{i}"][0], out_type=tf.float32, name=f"b{i}"
                )

                W = tf.Variable(W_tensor)
                b = tf.Variable(b_tensor)
                self.layers.append((W, b))
        else:
            for i in range(len(self.layer_sizes)):
                length_prev = self.layer_sizes[i - 1] if i > 0 else LAYER_SIZE_INPUT
                length_cur = self.layer_sizes[i]

                W = tf.Variable(
                    tf.random.normal([length_prev, length_cur], stddev=0.03),
                    dtype=tf.float32,
                )
                b = tf.Variable(
                    tf.random.normal([length_cur], stddev=0.03), dtype=tf.float32
                )

                self.layers.append((W, b))

        for W, b in self.layers:
            self.trainable_variables.append(W)
            self.trainable_variables.append(b)

    def apply(self, input):
        "Input must be in the form of a tf constant"
        x = input
        for W, b in self.layers:
            x = tf.matmul(x, W)
            x = tf.add(x, b)
            x = sigmoid(x)
        return x

    def copy(self):
        return Network(layer_sizes=self.layer_sizes,layers=[
            (
                tf.Variable(np.copy(W.numpy()),dtype=tf.float32),
                tf.Variable(np.copy(b.numpy()),dtype=tf.float32),
            ) for W,b in self.layers
        ])

    def serialize(self):
        features = {}
        for i in range(len(self.layers)):
            W, b = self.layers[i]
            features[f"W{i}"] = tf.train.Feature(
                bytes_list=tf.train.BytesList(value=[tf.io.serialize_tensor(W).numpy()])
            )
            features[f"b{i}"] = tf.train.Feature(
                bytes_list=tf.train.BytesList(value=[tf.io.serialize_tensor(b).numpy()])
            )
        return tf.train.Example(features=tf.train.Features(feature=features)).SerializeToString()
