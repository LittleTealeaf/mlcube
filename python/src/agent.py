from network import Network
from mlcube import PyReplay2x2
import tensorflow as tf
import random

class Agent:
    def __init__(self, name: str, replay: PyReplay2x2, hidden_layers: list[int]) -> None:
        self.name = name
        self.replay = replay
        self.network = Network(replay.observation_length, replay.action_size, hidden_layers = hidden_layers)
        self.target = Network(replay.observation_length, replay.action_size, clone_variables=self.network.layers)

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

