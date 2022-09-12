import json
import os
import tensorflow as tf
import numpy as np
from random import Random
from tensorflow import data as tfd

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

  def create_replay(self,count=1_000,epsilon=0.2,moves_min=1,moves_max=40):
    random = Random()
    entries_per_move = int(count / (moves_max - moves_min)) + 1
    epsilon_inverse = int(1 / 0.2)

    cubes = [Environment() for _ in range(entries_per_move)]
    for _ in range(moves_min):
      for cube in cubes:
        cube.apply_action(random.choice(ACTIONS))

    state_1 = np.zeros(shape=(count,1,9*6*6),dtype=np.float32)
    choice = np.zeros(shape=(count,1,1),dtype=np.float32)
    state_2 = np.zeros(shape=(count,1,9*6*6),dtype=np.float32)
    rewards = np.zeros(shape=(count,1,1),dtype=np.float32)

    for i in range(count):
      cube = cubes[i%len(cubes)]
      state_1[i] = cube.to_observations()
      value = self.network.apply(tf.constant(state_1[i],dtype=tf.float32))
      choice[i] = tf.argmax(value,axis=1).numpy() if i % epsilon_inverse != 0 else [random.randint(0,17)]
      cube.apply_action(ACTIONS[int(choice[i][0])])
      state_2[i] = cube.to_observations()
      cube.apply_action(random.choice(ACTIONS))
      rewards[i] = 1 if cube.is_complete() else 0

    return tfd.Dataset.from_tensor_slices((state_1,choice,state_2,rewards))


    # return tfd.Dataset.from_tensor_slices([tf.constant(i,dtype=tf.float32) for i in [state_1, choice, state_2, rewards]])

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
