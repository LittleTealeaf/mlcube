import tensorflow as tf
import numpy as np
from keras.initializers.initializers_v2 import VarianceScaling
from environment import ACTION_COUNT

LAYER_SIZE_INPUT = 9 * 6
LAYER_SIZE_OUTPUT = ACTION_COUNT


class Network:
    def __init__(self, layer_sizes: list[int], serialized=None, layers=None):
        self.layer_sizes = layer_sizes + [LAYER_SIZE_OUTPUT]
        """The array of layer sizes for the network"""
        self.trainable_variables: list[tf.Variable] = []
        """A list of all variables in the network that can be trained"""
        self.layers: list[tuple[tf.Variable, tf.Variable]] = []
        """The individual layers of the network. Each layer is represented by a weight variable and a bias variable"""

        if layers:
            self.layers = layers
        elif serialized:
            features = {}
            for i in range(len(layer_sizes)):
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
                length_prev = self.layer_sizes[i -
                                               1] if i > 0 else LAYER_SIZE_INPUT
                length_curr = self.layer_sizes[i]
                W = tf.Variable(VarianceScaling(
                    scale=2.0,
                    mode='fan_in',
                    distribution='truncated_normal'
                )(shape=(length_prev, length_curr), dtype=tf.float32))
                b = tf.Variable(VarianceScaling(
                    scale=2.0,
                    mode='fan_in',
                    distribution='truncated_normal'
                )(shape=(length_curr,), dtype=tf.float32))
                
                self.layers.append((W,b))

