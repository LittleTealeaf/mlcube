import tensorflow as tf

from keras.initializers.initializers_v2 import VarianceScaling


class Network:
    def __init__(
            self, observation_length: int, action_size: int, hidden_layers: list[int] | None = None, clone_variables: list[tuple[tf.Variable, tf.Variable]] | None = None
    ):
        self.layers: list[tuple[tf.Variable, tf.Variable]] = []
        self.trainable_variables: list[tf.Variable] = []
        self.hidden_layer_sizes = hidden_layers
        self.observation_length = observation_length
        self.action_size = action_size

        if clone_variables != None:
            for (w, b) in clone_variables:
                weight = tf.Variable(w.numpy())
                bias = tf.Variable(b.numpy())

                self.trainable_variables.append(weight)
                self.trainable_variables.append(bias)
                self.layers.append((weight, bias))



        elif hidden_layers != None:
            for i in range(len(hidden_layers) + 1):
                length_prev = hidden_layers[i - 1] if i > 0 else self.observation_length
                length_cur = (
                    hidden_layers[i] if i < len(hidden_layers) else self.action_size
                )

                W = tf.Variable(
                    VarianceScaling(
                        scale=1.0, mode="fan_in", distribution="truncated_normal"
                    )(shape=(length_prev, length_cur), dtype=tf.float32)
                )

                b = tf.Variable(
                    VarianceScaling(
                        scale=1.0, mode="fan_in", distribution="truncated_normal"
                    )(shape=(length_cur,), dtype=tf.float32)
                )

                self.layers.append((W, b))
                self.trainable_variables.append(W)
                self.trainable_variables.append(b)
    
    def apply(self, values):
        observations = tf.constant(values, dtype=tf.float32, shape=(1,144))
        for weight, bias in self.layers:
            observations = tf.matmul(observations, weight)
            observations = tf.add(observations, bias)

        return observations

a = Network(3*3*6*6, 18, [300,200,100])