import json
import os
import tensorflow as tf
import numpy as np
from random import Random
from keras.optimizers import SGD

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
      values = self.network.apply(tf.constant(np.array(env.to_observations()),dtype=tf.float32))
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

  def create_replay(self,replay_size=1_000,epsilon=0.2,moves_min=1,moves_max=40):
    random = Random()
    entries_per_move = int(replay_size / (moves_max - moves_min)) + 1
    epsilon_inverse = int(1 / 0.2)

    cubes = [Environment() for _ in range(entries_per_move)]
    for _ in range(moves_min):
      for cube in cubes:
        cube.apply_action(random.choice(ACTIONS))

    state_1 = np.zeros(shape=(replay_size,1,9*6*6),dtype=np.float32)
    choice = np.zeros(shape=(replay_size,1,1),dtype=np.float32)
    state_2 = np.zeros(shape=(replay_size,1,9*6*6),dtype=np.float32)
    rewards = np.zeros(shape=(replay_size,1,1),dtype=np.float32)

    for i in range(replay_size):
      cube = cubes[i%len(cubes)]
      state_1[i] = cube.to_observations()
      value = self.network.apply(tf.constant(state_1[i],dtype=tf.float32))
      choice[i] = tf.argmax(value,axis=1).numpy() if i % epsilon_inverse != 0 else [random.randint(0,17)]
      cube.apply_action(ACTIONS[int(choice[i][0])])
      state_2[i] = cube.to_observations()
      cube.apply_action(random.choice(ACTIONS))
      rewards[i] = cube.reward()

    return (state_1,choice,state_2,rewards)



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

  def train_replay(self,replay: tuple[(np.ndarray,np.ndarray,np.ndarray,np.ndarray)]):
    "Replay must be a tensor slice dataset"
    state_1, choice, state_2, rewards = replay



    with tf.GradientTape() as tape:

      tape.watch(self.network.trainable_variables)

      t_state_1 = tf.constant(state_1,dtype=tf.float32)
      t_choice = tf.constant(choice,dtype=tf.int64)
      t_state_2 = tf.constant(state_2,dtype=tf.float32)
      t_rewards = tf.constant(rewards,dtype=tf.float32)

      t_state_1_output = self.network.apply(t_state_1)
      t_state_1_choice_q = tf.gather(t_state_1_output,t_choice,batch_dims=2)

      t_state_2_output = self.target.apply(t_state_2)
      t_state_2_choices = tf.argmax(t_state_2_output,2)
      t_state_2_choices_q = tf.gather(t_state_2_output,t_state_2_choices,batch_dims=2)

      t_state_2_choices_q = tf.multiply(t_state_2_choices_q,0.75)

      t_target_q = tf.add(t_state_2_choices_q, tf.reshape(t_rewards, (t_rewards.shape[0],1)))

  


      t_predicted_q = tf.reshape(t_state_1_choice_q,(t_state_1_choice_q.shape[0],1))

      t_abs_loss = tf.subtract(t_target_q,t_predicted_q)

      t_loss = tf.square(t_abs_loss)

      gradient = tape.gradient(t_loss,self.network.trainable_variables)

      return t_loss, gradient

  def run_cycle(self,replay_size=1000,epsilon=0.2,moves_min=1,moves_max=40, learning_rate=0.1):
    replay = self.create_replay(replay_size=replay_size,epsilon=epsilon,moves_min=moves_min,moves_max=moves_max)

    loss, gradient = self.train_replay(replay)

    loss_avg = float(tf.math.reduce_mean(loss).numpy())

    optimizer = SGD(learning_rate=learning_rate)
    optimizer.apply_gradients(zip(gradient,self.network.trainable_variables))

    self.epochs.append({
      'epoch': self.get_epoch(),
      'loss': loss_avg
    })

    return loss_avg, loss_avg**0.5
