import json
import math
from multiprocessing import Pool
import os
import time
import tensorflow as tf
import numpy as np
from random import Random
from keras.optimizers import SGD
import functools

from src.network import *
from src.environment import REWARDS, Environment, ACTIONS, create_scrambled_env, action_from_choice

def pool_apply_epsilon(args):
  choice,epsilon = args
  random = Random()
  if random.random() < epsilon:
    return choice
  else:
    return random.randint(0,17)

def pool_apply_actions(args):
  env, action = args
  env.apply_action(action)
  return env

def pool_get_rewards(args):
  env, rewards = args
  hash = env.hash()
  return rewards[hash] if hash in rewards else 0

class Agent:
  def __init__(self, layer_sizes: list[int], directory: str):
    "Create a new agent"
    self.directory = directory
    self.network: Network = None
    self.target: Network = None
    self.layer_sizes = layer_sizes
    self.epochs = []
    self.evaluations = []

    if os.path.exists(directory):
      try:
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
    max_reward = 0.0

    while (not env.is_complete()) and move_count < max_moves:
      max_reward = max(max_reward,env.reward())
      move_count = move_count + 1
      values = self.network.apply(tf.constant(env.to_observations(),dtype=tf.float32))
      move = ACTIONS[tf.argmax(values).numpy()[0]]
      env.apply_action(move)



    solved = env.is_complete()
    final_reward = env.reward()

    self.evaluations.append({
      'epoch': self.get_epoch(),
      'moves': move_count,
      'max_reward': max_reward,
      'final_reward': final_reward
    })

    return (move_count, solved, max_reward, final_reward)

  def get_epoch(self):
    "Get the current epoch"
    return len(self.epochs)

  def save(self):
    "Save the network and target to disk"
    serialized_network = self.network.serialize()
    serialized_target = self.target.serialize()

    tf.io.write_file(os.path.join(self.directory,"network"),serialized_network)
    tf.io.write_file(os.path.join(self.directory,"target"),serialized_target)

    with open(os.path.join(self.directory,"epochs.json"),'w') as file:
      file.write(json.dumps(self.epochs))

    with open(os.path.join(self.directory,"evaluations.json"),'w') as file:
      file.write(json.dumps(self.evaluations))

  def update_target(self):
    "Update the target network to match the current network"
    self.target = self.network.copy()



  # def run_cycle(self,replay_size=1000,epsilon=0.2,moves_min=1,moves_max=40, learning_rate=0.1, gamma = 0.5):
  #   "Run a cycle of training and evaluation"

  #   # This is what takes up most of the time
  #   replay = self.create_replay_deprecated(replay_size=replay_size,epsilon=epsilon,moves_min=moves_min,moves_max=moves_max)

  #   loss, gradient, t_rewards = self.train_replay_deprecated(replay,gamma=gamma)

  #   avg_reward = float(tf.math.reduce_mean(t_rewards).numpy())

  #   loss_avg = float(tf.math.reduce_mean(loss).numpy())

  #   optimizer = SGD(learning_rate=learning_rate)
  #   optimizer.apply_gradients(zip(gradient,self.network.trainable_variables))


  #   self.epochs.append({
  #     'epoch': self.get_epoch(),
  #     'loss': loss_avg,
  #     'reward': avg_reward
  #   })

  #   return loss_avg, loss_avg**0.5,avg_reward

  # def run_multithread_cycle(self,replay_size=1000,epsilon=0.2,moves_min=1,moves_max=40,learning_rate=0.1,gamma=0.5,pool=Pool(1),rewards=None):
  #   with tf.GradientTape() as tape:
  #     assert moves_max > moves_min

  #     tape.watch(self.network.trainable_variables)

  #     moves_diff = moves_max - moves_min
  #     replay_scramble_depths = [i%moves_diff + moves_min for i in range(replay_size)]
  #     replay_environments = pool.map(create_scrambled_env,replay_scramble_depths)
  #     del replay_scramble_depths
  #     state_1_observations = pool.map(Environment.to_observations,replay_environments)
  #     tf_state_1 = tf.constant(np.array(state_1_observations),dtype=tf.float32)
  #     tf_state_1_output = self.network.apply(tf_state_1)
  #     tf_state_1_choices = tf.argmax(tf_state_1_output,2)
  #     np_state_1_choices = tf_state_1_choices.numpy()
  #     del tf_state_1_choices

  #     cubes_per_random = math.ceil(1 / epsilon)
  #     random = Random()
  #     for i in range(0,replay_size,cubes_per_random):
  #       np_state_1_choices[i] = [random.randint(0,17)]

  #     actions = pool.map(action_from_choice,np_state_1_choices)
  #     action_pairs = [(replay_environments[i],actions[i]) for i in range(replay_size)]
  #     replay_environments = pool.map(pool_apply_actions,action_pairs)
  #     state_2_observations = pool.map(Environment.to_observations,replay_environments)

  #     rewards = pool.map(pool_get_rewards,[(i,REWARDS) for i in replay_environments])

  #     del replay_environments

  #     tf_rewards = tf.constant(np.array(rewards),dtype=tf.float32)
  #     del rewards

  #     tf_state_2 = tf.constant(np.array(state_2_observations),dtype=tf.float32)
  #     tf_state_2_output = self.target.apply(tf_state_2)


# https://czxttkl.com/2015/09/28/python-multiprocessing-map-function-with-shared-memory-object-as-additional-parameter/





      # replay_environments = pool.map(create_scrambled_env,replay_scramble_depths)
      # tf_state_1 = tf.constant(np.array(replay_environments),dtype=tf.float32)
      # tf_state_1_output = self.network.apply(tf_state_1)
      # tf_state_1_choice = tf.argmax(tf_state_1_output,2)


  # def create_replay_deprecated(self,replay_size=1_000,epsilon=0.2,moves_min=1,moves_max=40, pool = None):
  #   "Create a replay of random moves"

  #   state_1 = np.zeros(shape=(replay_size,1,9*6*6),dtype=np.float32)
  #   choice = np.zeros(shape=(replay_size,1,1),dtype=np.float32)
  #   state_2 = np.zeros(shape=(replay_size,1,9*6*6),dtype=np.float32)
  #   rewards = np.zeros(shape=(replay_size,1,1),dtype=np.float32)

  #   random = Random()
  #   entries_per_move = int(replay_size / (moves_max - moves_min)) + 1
  #   cubes = [Environment() for _ in range(entries_per_move)]
  #   for _ in range(moves_min):
  #     for cube in cubes:
  #       cube.apply_action(random.choice(ACTIONS))
  #   for i in range(replay_size):
  #     cube = cubes[i%len(cubes)]
  #     state_1[i] = cube.to_observations_deprecated()
  #     value = self.network.apply(tf.constant(state_1[i],dtype=tf.float32))
  #     choice[i] = tf.argmax(value,axis=1).numpy() if random.random() >= epsilon else [random.randint(0,17)]
  #     cube.apply_action(ACTIONS[int(choice[i][0])])
  #     state_2[i] = cube.to_observations_deprecated()
  #     cube.apply_action(random.choice(ACTIONS))
  #     rewards[i] = cube.reward()


  #   return (state_1,choice,state_2,rewards)


  # def train_replay_deprecated(self,replay: tuple[(np.ndarray,np.ndarray,np.ndarray,np.ndarray)], gamma = 0.5):
  #   "Replay must be a tensor slice dataset"
  #   state_1, choice, state_2, rewards = replay

  #   assert gamma >= 0 and gamma <= 1

  #   with tf.GradientTape() as tape:

  #     tape.watch(self.network.trainable_variables)

  #     t_state_1 = tf.constant(state_1,dtype=tf.float32)
  #     t_choice = tf.constant(choice,dtype=tf.int64)
  #     t_state_2 = tf.constant(state_2,dtype=tf.float32)
  #     t_rewards = tf.constant(rewards,dtype=tf.float32)

  #     t_state_1_output = self.network.apply(t_state_1)
  #     t_state_1_choice_q = tf.gather(t_state_1_output,t_choice,batch_dims=2)

  #     t_state_2_output = self.target.apply(t_state_2)
  #     t_state_2_choices = tf.argmax(t_state_2_output,2)
  #     t_state_2_choices_q = tf.gather(t_state_2_output,t_state_2_choices,batch_dims=2)

  #     t_state_2_choices_q_scaled = tf.multiply(t_state_2_choices_q, gamma)

  #     t_rewards = tf.reshape(t_rewards, (t_rewards.shape[0],1))

  #     t_rewards_scaled = tf.multiply(t_rewards,1 - gamma)

  #     t_target_q = tf.add(t_state_2_choices_q_scaled, t_rewards_scaled)

  #     t_predicted_q = tf.reshape(t_state_1_choice_q,(t_state_1_choice_q.shape[0],1))

  #     t_abs_loss = tf.subtract(t_target_q,t_predicted_q)

  #     t_loss = tf.square(t_abs_loss)

  #     gradient = tape.gradient(t_loss,self.network.trainable_variables)

  #     return t_loss, gradient, t_rewards
