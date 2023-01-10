import tensorflow as tf
from keras.activations import relu
from keras.initializers.initializers_v2 import VarianceScaling

import numpy as np

LAYER_SIZE_INPUT = 9 * 6 * 6
LAYER_SIZE_OUTPUT = 18

class NetworkType:
    def __init__(self):
        self.layers: list[tuple[tf.Variable,tf.Variable]] = []
        self.trainable_variables: list[tf.Variable] = []


class Network(NetworkType):
    def __init__(self, layer_sizes: list[int], serialized=None, layers: list[tuple[tf.Variable, tf.Variable]] | None = None, output_size=18):
        """
        Creates a new network

        Parameters
        ----------
        layer_sizes: list[int]
            A list containing the sizes that each of the internal layer should be. The input and output layer should not be included, as those will be automatically generated.

        serialized = None
            The serialized version of the network. Used when deserializing a serialized network

        layers: list[tuple[tf.Variable, tf.Variable]] | None = None
            A predefined list of layers to use as the network. This is primarily used when cloning a network

        output_size: int = 18
            The number of output variables that the network should output
        """
        self.layer_sizes = layer_sizes + [output_size]
        self.trainable_variables: list[tf.Variable] = []
        self.layers: list[tuple[tf.Variable,tf.Variable]] = []

        if layers:
            self.layers: list[tuple[tf.Variable, tf.Variable]] = layers
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
                W = tf.Variable(VarianceScaling(
                    scale=1.0,
                    mode='fan_in',
                    distribution='truncated_normal'
                )(shape=(length_prev,length_cur),dtype=tf.float32))

                b = tf.Variable(VarianceScaling(
                    scale=1.0,
                    mode='fan_in',
                    distribution='truncated_normal'
                )(shape=(length_cur,),dtype=tf.float32))

                self.layers.append((W, b))

        for W, b in self.layers:
            self.trainable_variables.append(W)
            self.trainable_variables.append(b)

    @tf.function
    def apply(self, input):
        "Input must be in the form of a tf constant"
        print(f"Tracing apply function with input {input.shape}")
        if len(input.shape) == 1:
            x = tf.reshape(input,(1,9*6*6))
        else:
            x = input
        for i in range(len(self.layers)):
            W,b = self.layers[i]
            x = tf.matmul(x,W)
            x = tf.add(x,b)
            if i < len(self.layers) - 1:
                x = relu(x)
        return x

    def copy(self):
        return Network(layer_sizes=self.layer_sizes,layers=[
            (
                tf.Variable(np.copy(W.numpy()),dtype=tf.float32),
                tf.Variable(np.copy(b.numpy()),dtype=tf.float32),
            ) for (W,b) in self.layers
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

    def set(self,other: NetworkType):
        for i in range(len(self.layers)):
            W,b = self.layers[i]
            Wp, bp = other.layers[i]
            W.assign(Wp.numpy())
            b.assign(bp.numpy())
