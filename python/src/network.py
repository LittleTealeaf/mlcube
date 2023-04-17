import tensorflow as tf
from keras.activations import relu
from keras.initializers.initializers import VarianceScaling
from database import Database


class Network:
    def __init__(
        self,
        observation_length: int,
        action_size: int,
        hidden_layers: list[int] | None = None,
        clone_variables: list[tuple[tf.Variable, tf.Variable]] | None = None,
        from_database: tuple[Database, int] | None = None,
    ):
        self.layers: list[tuple[tf.Variable, tf.Variable]] = []
        self.trainable_variables: list[tf.Variable] = []
        self.hidden_layer_sizes = hidden_layers
        self.observation_length = observation_length
        self.action_size = action_size

        if clone_variables != None:
            for w, b in clone_variables:
                weight = tf.Variable(w.numpy())
                bias = tf.Variable(b.numpy())

                self.trainable_variables.append(weight)
                self.trainable_variables.append(bias)
                self.layers.append((weight, bias))

        elif from_database != None:
            database, network_id = from_database
            for layer in range(len(hidden_layers) + 1):
                weight, bias = database.get_network_layer(network_id, layer)
                W = tf.Variable(weight, dtype=tf.float32)
                b = tf.Variable(bias, dtype=tf.float32)

                self.trainable_variables.append(W)
                self.trainable_variables.append(b)
                self.layers.append((W, b))

        elif hidden_layers != None:
            for i in range(len(hidden_layers) + 1):
                length_prev = hidden_layers[i - 1] if i > 0 else self.observation_length
                length_cur = (
                    hidden_layers[i] if i < len(hidden_layers) else self.action_size
                )

                W = tf.Variable(
                    VarianceScaling(
                        scale=0.1, mode="fan_in", distribution="truncated_normal"
                    )(shape=(length_prev, length_cur), dtype=tf.float32)
                )

                b = tf.Variable(
                    VarianceScaling(
                        scale=0.1, mode="fan_in", distribution="truncated_normal"
                    )(shape=(length_cur,), dtype=tf.float32)
                )

                self.layers.append((W, b))
                self.trainable_variables.append(W)
                self.trainable_variables.append(b)

    def apply(self, values, count = 1):
        observations = tf.constant(values, dtype=tf.float32, shape=(count, 144))
        for index, (weight, bias) in enumerate(self.layers):
            observations = tf.matmul(observations, weight)
            observations = tf.add(observations, bias)

            # if index != len(self.layers) - 1:
            #     observations = relu(observations)

        return observations

    def save_to_database(
        self, database: Database, modelid: int, is_target: bool = False
    ):
        network_id = database.create_network(modelid, is_target)
        for i, (w, b) in enumerate(self.layers):
            database.insert_weights(network_id, i, w.numpy())
            database.insert_bias(network_id, i, b.numpy())


a = Network(3 * 3 * 6 * 6, 18, [300, 200, 100])
