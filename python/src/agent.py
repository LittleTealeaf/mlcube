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
