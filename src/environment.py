from random import Random
import numpy as np
import tensorflow as tf

class Action:
    def __init__(self, name: str, loops: list[list[int]], two=False, prime=False):
        self.name = name
        self.matrix = np.identity(9 * 6,dtype=np.int8)

        for loop in loops:
          initial = np.copy(self.matrix[loop[0]])
          for i in range(len(loop) - 1):
            self.matrix[loop[i]] = self.matrix[loop[i+1]]
          self.matrix[loop[-1]] = initial

        del initial

        if two:
            self.matrix = self.matrix @ self.matrix

        if prime:
            self.matrix = self.matrix @ self.matrix @ self.matrix

    def apply(self,state):
      return state @ self.matrix



def create_moves(name: str, loops: list[list[int]]):
    return [
        Action(name, loops),
        Action(f"{name}P", loops, prime=True),
        Action(f"{name}2", loops, two=True),
    ]

ACTIONS = [
    move
    for moves in [
        create_moves(
            "R",
            [
                [20, 2, 42, 47],
                [23, 5, 39, 50],
                [26, 8, 36, 53],
                [27, 29, 35, 33],
                [28, 32, 34, 30],
            ],
        ),
        create_moves(
            "U",
            [
                [20, 11, 38, 29],
                [19, 10, 37, 28],
                [18, 9, 36, 27],
                [8, 6, 0, 2],
                [7, 3, 1, 5],
            ],
        ),
        create_moves(
            "L",
            [
                [18, 45, 44, 0],
                [21, 48, 41, 3],
                [24, 51, 38, 6],
                [11, 17, 15, 9],
                [14, 16, 12, 10],
            ],
        ),
        create_moves(
            "D",
            [
                [24, 33, 42, 15],
                [25, 34, 43, 16],
                [26, 35, 44, 17],
                [45, 47, 53, 51],
                [46, 50, 52, 48],
            ],
        ),
        create_moves(
            "F",
            [
                [6, 27, 47, 17],
                [7, 30, 46, 14],
                [8, 33, 45, 11],
                [18, 20, 26, 24],
                [19, 23, 25, 21],
            ],
        ),
        create_moves(
            "B",
            [
                [36, 38, 44, 42],
                [37, 41, 43, 39],
                [29, 0, 15, 53],
                [32, 1, 12, 52],
                [35, 2, 9, 51],
            ],
        ),
    ]
    for move in moves
]


# def create_environment(scramble_depth=None):
#     if not scramble_depth:
#       return np.fromfunction(lambda i: i // 9,(9*6,))
#     else:
#       return scramble(create_environment(),)

# def scramble(environment, count = 100, random = Random()):
#   for _ in range(count):
#     environment = random.choice(ACTIONS).apply(environment)
#   return environment

def create_environment(scramble_depth=0,random=Random()):
  env = np.fromfunction(lambda i: i // 9, (9*6,))
  for _ in range(scramble_depth):
    env = random.choice(ACTIONS).apply(env)
  return env

def env_to_observations(environment):
    obs = np.zeros((9*6*6),dtype=np.float32)
    for i in range(9*6):
        index = int(environment[i])
        obs[i * 6 + index] = 1
    return obs

def env_to_obs_tf(env):
    return tf.constant(env_to_observations(env),dtype=tf.float32)

def hash_env(env):
    return hash(env.tostring())


def calculate_rewards(depth=8,decay=0.9,base = 1):
    rewards = {}
    buffer = [create_environment()]
    for i in range(depth):
        print(f'Calculating depth {i} with length {len(buffer)}')
        tmp_buffer = []
        for env in buffer:
            ehash = hash_env(env)
            if ehash not in rewards:
                rewards[ehash] = base * decay ** i
                if i < depth - 1:
                    for action in ACTIONS:
                        tmp_buffer.append(action.apply(env))
        buffer = tmp_buffer
    return rewards

def get_reward(env,rewards={}):
    ehash = hash_env(env)
    return rewards[ehash] if ehash in rewards else 0

def env_is_complete(env):
    for i in range(9 * 6):
        if env[i] != i // 9:
            return False
    return True



COUNT_ACTIONS = len(ACTIONS)
