from multiprocessing import Pool
from random import Random
from types import NoneType
import numpy as np
import hashlib




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

REWARDS = {}

# def create_observation_set(i):
#     val = [0] * 6
#     val[i] = 1
#     return val


class Environment:
    def __init__(self, observations: list[list[int]] = None):
        if observations:
            self.state = np.array([0] * (9 * 6),dtype=np.int8)
            for i in range(len(observations[0])):
                self.state[i//6] += observations[0][i] * (i%6)
        else:
            self.reset()
        self.observation_cache = None



    def reset(self):
        self.state = np.array([i // 9 for i in range(9 * 6)],dtype=np.int8)
        return self

    def apply_action(self,action: Action):
        self.observation_cache = None
        self.state = action.apply(self.state)
        return self

    def is_complete(self):
      for i in range(9 * 6):
        if self.state[i] != i // 9:
          return False
      return True

    # def to_observations_deprecated(self):
    #   # I think this works
    #   return [[
    #     value for position in [
    #       create_observation_set(i) for i in self.state
    #     ] for value in position
    #   ]]

    def to_observations(self,save_cache=True, use_cache=True):
        if use_cache and not type(self.observation_cache) == NoneType:
            return self.observation_cache

        array = np.zeros((9 * 6 * 6,),dtype=np.float32)
        for i in range(9 * 6):
            array[i * 6 + self.state[i]] = 1
        if save_cache:
            self.observation_cache = array
        return array

    def scramble(self,count: int = 100):
        random = Random()
        for _ in range(count):
            self.apply_action(random.choice(ACTIONS))
        return self

    def copy(self):
        env = Environment()
        env.state = np.copy(self.state)
        return env

    def hash(self):
        return observation_to_hash(self.to_observations())

    def reward(self, rewards):
        hash = self.hash()
        try:
            return rewards[hash]
        except KeyError:
            return 0


def calculate_rewards(depth=8,decay=0.8, pool: Pool=None):
    if pool != None:
        rewards = {}
        buffer = [Environment()]
        for i in range(depth):
            print(f'Calculating depth {i} with length {len(buffer)}')
            values = pool.map(Environment.hash,buffer)
            for item in values:
                rewards[item] = decay ** i

        return rewards


    else:
        rewards = {}
        buffer = [Environment()]
        for i in range(depth):
            print(f"Calculating depth {i} with length {len(buffer)}")
            tmp_buffer = []
            for env in buffer:
                hash = env.hash()
                if hash not in rewards:
                    rewards[hash] = decay ** i
                    if i < depth - 1:
                        for action in ACTIONS:
                            tmp_buffer.append(env.copy().apply_action(action))
            buffer = tmp_buffer
        print(f"Reward list is {len(rewards)} items long")
        return rewards

def create_scrambled_environment(depth):
    env = Environment()
    env.scramble(depth)
    return env


LEN_STATE = Environment().state.shape[0]
LEN_OBSERVATIONS = Environment().to_observations(use_cache=False,save_cache=False).shape[0]


def observation_to_hash(obs: np.ndarray[(LEN_OBSERVATIONS,)]):
    # convert observations to the state
    hashed = np.zeros((3,))

    for i in range(LEN_OBSERVATIONS):
        if obs[i] == 1:
            index = (i//6)//18
            hashed[index] *= 6
            hashed[index] += i%6
    return hashed.mean()




COUNT_ACTIONS = len(ACTIONS)

HASH_COMPLETE = Environment().hash()
