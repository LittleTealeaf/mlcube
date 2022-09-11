import json
import os
import tensorflow as tf
from random import Random

from src.network import *
from src.environment import Environment, ACTIONS


class Agent:
  def __init__(self, layer_sizes: list[int], directory: str):
    self.directory = directory
    self.network: Network = None
    self.target: Network = None
    self.layer_sizes = layer_sizes
    self.epochs = []
    self.evaluations = []

    if os.path.exists(directory):
      try:
        ...
        # Epoch History
        with open(os.path.join(directory,"epochs.json")) as file:
          self.epochs = json.load(file)

        # Evaluation history
        with open(os.path.join(directory,"evaluations.json")) as file:
          self.evaluations = json.load(file)

        # network and target
        network_data = tf.io.read_file(os.path.join(directory,"network"))
        target_data = tf.io.read_file(os.path.join(directory,"target"))

        self.network = Network(layer_sizes,serialized=network_data)
        self.target = Network(layer_sizes,serialized=target_data)

      except Exception as e:
        print(f"Tried opening and failed: {str(e)}")

    if not self.network:
      self.network = Network(layer_sizes)
      self.update_target()

  def evaluate_network(self,max_moves=10_000, scramble_depth=100):
    random = Random()
    env = Environment()
    for _ in range(scramble_depth):
      env.apply_action(random.choice(ACTIONS))

    move_count = 0

    while (not env.is_complete()) and move_count < max_moves:
      move_count = move_count + 1
      values = self.network.apply(env.to_observations())
      move = ACTIONS[tf.argmax(values).numpy()[0]]
      env.apply_action(move)

    solved = env.is_complete()

    self.evaluations.append({
      'epoch': self.get_epoch(),
      'solved': solved,
      'moves': move_count
    })

    return (move_count, solved)

  def get_epoch(self):
    return len(self.epochs)

  

  def save(self):
    serialized_network = self.network.serialize()
    serialized_target = self.target.serialize()

    tf.io.write_file(os.path.join(self.directory,"network"),serialized_network)
    tf.io.write_file(os.path.join(self.directory,"target"),serialized_target)

    with open(os.path.join(self.directory,"epochs.json"),'w') as file:
      file.write(json.dumps(self.epochs))

    with open(os.path.join(self.directory,"evaluations.json"),'w') as file:
      file.write(json.dumps(self.evaluations))

  def update_target(self):
    self.target = self.network.copy()
