from network import Network
from mlcube import PyReplay2x2
import tensorflow as tf
import random
from database import Database

class Agent:
    def __init__(self, name: str, replay: PyReplay2x2, hidden_layers: list[int], database: Database, max_saved_network: int = 1) -> None:
        self.name = name
        self.replay = replay
        self.database = database

        self.model_id = database.get_model_id(name)
        if self.model_id == None:
            self.model_id = database.create_model(name, replay.get_name())


        network_id = database.get_latest_network(self.model_id, False)
        if network_id != None:
            self.network = Network(replay.observation_length, replay.action_size, hidden_layers = hidden_layers, from_database=(database, network_id))
        else:
            self.network = Network(replay.observation_length, replay.action_size, hidden_layers = hidden_layers)
            self.network.save_to_database(database, self.model_id, False)

        target_id = database.get_latest_network(self.model_id, True)
        if target_id != None:
            self.target = Network(replay.observation_length, replay.action_size, hidden_layers = hidden_layers, from_database=(database, target_id))
        else:
            self.target = Network(replay.observation_length, replay.action_size, clone_variables=self.network.layers)
            self.target.save_to_database(database, self.model_id, True)

    def step_experience(self, epsilon: float):
        if random.uniform(0,1) < epsilon:
            self.replay.apply_action(int(random.uniform(0,self.replay.action_size)))
        else:
            values = self.network.apply(self.replay.get_observations())
            choice = tf.argmax(values, axis=1)
            self.replay.apply_action(choice.numpy()[0])

    def train(self, sample_size: int, learning_rate: float, gamma: float):
        (first_state, action, reward, next_state) = self.replay.sample_replay(sample_size)
        first_state = tf.constant(first_state);
        action = tf.constant(action)
        reward = tf.constant(reward)
        next_state = tf.constant(next_state)


        with tf.GradientTape() as tape:

            tape.watch(self.network.trainable_variables)

            output_1 = self.network.apply(first_state)
            output_1_gathered = tf.gather(output_1, action, batch_dims=1)
            output_2 = self.target.apply(next_state)
            output_2_gathered = tf.reduce_max(output_2, axis=1)

            output_2_gathered_scaled = tf.multiply(output_2_gathered, gamma)

            loss_raw = tf.reshape(output_2_gathered_scaled, (output_2_gathered_scaled.shape[0], 1)) - output_1_gathered - reward

            loss = tf.math.square(loss_raw)

            loss_mean = tf.reduce_mean(loss)

            gradient = tape.gradient(loss_mean, self.network.trainable_variables)

            optimizer = tf.keras.optimizers.SGD(learning_rate=learning_rate)

            optimizer.apply_gradients(zip(gradient, self.network.trainable_variables))


    def save(self):
        self.network.save_to_database(self.database, self.model_id, False)
        self.target.save_to_database(self.database,self.model_id, True)
        self.database.keep_latest_networks(self.model_id, 1, is_target=False)
        self.database.keep_latest_networks(self.model_id, 1, is_target=True)
