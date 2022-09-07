import json
import os
from random import Random

import numpy as np
import tensorflow as tf
from keras.optimizers import SGD

from network import Network, MOVES



def create_cube():
    state = np.zeros(9 * 6 * 6, dtype=np.float32)
    for i in range(0, 9 * 6 * 6, 6):
        state[i + i // (9 * 6)] = 1
    return tf.constant(state, dtype=tf.float32, shape=(1, 324))


def reward(state):
    return 1 if tf.math.reduce_all(tf.math.equal(state, create_cube())) else 0


def exponential_decay(initial, index, decay_rate, decay_interval):
    return initial * (decay_rate ** (index // decay_interval))


class Agent:
    def __init__(self, layer_sizes: list[int], directory: str = "./agent"):
        self.network = None
        self.dir = directory
        self.eval_history = []
        self.epoch = 0

        if os.path.exists(directory):
            try:
                with open("/".join([self.dir, 'config.json'])) as file:
                    values = json.load(file)
                    self.eval_history = values['eval_history']
                    self.epoch = values['epoch_count']

                network_data = tf.io.read_file("/".join([self.dir, 'agent']))
                self.network = Network(layer_sizes, serialized=network_data)
            except Exception as e:

                print(f"Tried opening and failed: {str(e)}")

        if not self.network:
            self.network: Network = Network(layer_sizes)

        self.update_target()

    def update_target(self):
        self.target: Network = self.network.copy()

    def create_replay(self, count, epsilon=0.5):
        random = Random()
        state_1_list = []
        for i in range(count):
            cube = create_cube()
            for _ in range(1, i % 40 + 1):
                cube = random.choice(MOVES).apply(cube)
            state_1_list.append(cube)

        state_1 = tf.constant(np.array(state_1_list))
        state_1_outputs = self.network.apply(state_1)
        state_1_choices = tf.argmax(state_1_outputs, 2)
        state_1_choices = tf.map_fn(
            lambda i: i if random.random() > epsilon else np.array([random.choice(MOVES).index]), state_1_choices)

        state_2_list = [MOVES[state_1_choices[i][0]].apply(state_1[i]) for i in range(count)]

        reward_1 = tf.constant(np.array([reward(state) for state in state_2_list]), dtype=tf.float32)

        state_2 = tf.constant(np.array(state_2_list))

        return state_1, state_1_choices, reward_1, state_2

    def train_replay(self, replays):
        with tf.GradientTape() as tape:
            # tape.watch(self.network.trainable_variables)

            state_1, state_1_choices, reward_1, state_2 = replays

            state_1_output = self.network.apply(state_1)
            state_1_choice_q = tf.gather(state_1_output, state_1_choices, batch_dims=2)

            state_2_output = self.target.apply(state_2)
            state_2_choices = tf.argmax(state_2_output, 2)
            state_2_choices_q = tf.gather(state_2_output, state_2_choices, batch_dims=2)

            state_2_choices_q = tf.multiply(state_2_choices_q, 0.75)

            # scale the choice Qs

            target_q = tf.add(state_2_choices_q, tf.reshape(reward_1, (reward_1.shape[0], 1)))

            predicted_q = state_1_choice_q

            abs_loss = tf.subtract(target_q, predicted_q)

            loss = tf.square(abs_loss)

            gradient = tape.gradient(loss, self.network.trainable_variables)

            return loss, gradient




    def run_epoch(self, replay_size=1000, EPSILON=0.5, learning_rate=0.1):

        replay = self.create_replay(replay_size, epsilon=EPSILON)
        loss, gradient = self.train_replay(replay)
        loss_avg = tf.math.reduce_mean(loss)
        optimizer = SGD(learning_rate=learning_rate)
        optimizer.apply_gradients(zip(gradient, self.network.trainable_variables))
        self.epoch = self.epoch + 1
        return loss_avg

    def evaluate_network(self):
        random = Random()
        cube = create_cube()
        for _ in range(100):
            cube = random.choice(MOVES).apply(cube)

        move_count = 0

        while reward(cube) != 1 and move_count < 10000:
            move_count = move_count + 1
            values = self.network.apply(cube)
            move = MOVES[tf.argmax(values).numpy()[0]]
            cube = move.apply(cube)

        self.eval_history.append({
            'x': self.epoch,
            'y': move_count
        })

        return move_count

    def save(self):
        serialized = self.network.serialize()
        tf.io.write_file("/".join([self.dir, 'agent']), serialized.SerializeToString())
        with open("/".join([self.dir, 'config.json']), 'w') as file:
            file.write(json.dumps({
                'epoch_count': self.epoch,
                'eval_history': self.eval_history
            }))


agent = Agent(layer_sizes=[100, 50], directory="./agents/1")
target_interval = 500
eval_interval = 500
save_interval = target_interval

while True:
    avg_loss = agent.run_epoch(replay_size=10_000, EPSILON=0.2, learning_rate=exponential_decay(exponential_decay(1,agent.epoch%500,0.9,3),agent.epoch // 500, 0.9, 1))
    print(f'Epoch {agent.epoch}\tAverage Loss \t{avg_loss} \t({avg_loss ** (0.5)})')

    if agent.epoch % target_interval == 0:
        agent.update_target()

    if agent.epoch % eval_interval == 0:
        eval_result = agent.evaluate_network()
        print(f"Evaluated at {agent.epoch}: {eval_result}")

    if agent.epoch % save_interval == 0:
        agent.save()
