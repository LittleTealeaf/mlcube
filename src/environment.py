from random import Random
import numpy as np

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

def create_observation_set(i):
    val = [0] * 6
    val[i] = 1
    return val


class Environment:
    def __init__(self, observations: list[list[int]] = None):
        if observations:
            self.state = np.array([0] * (9 * 6),dtype=np.int8)
            for i in range(len(observations[0])):
                self.state[i//6] += observations[0][i] * (i%6)
        else:
            self.reset()


    def reset(self):
        self.state = np.array([i // 9 for i in range(9 * 6)],dtype=np.int8)
        return self

    def apply_action(self,action: Action):
      self.state = action.apply(self.state)
      return self

    def is_complete(self):
      for i in range(9 * 6):
        if self.state[i] != i // 9:
          return False
      return True

    def to_observations_deprecated(self):


      # I think this works
      return [[
        value for position in [
          create_observation_set(i) for i in self.state
        ] for value in position
      ]]

    def to_observations(self):
        array = np.zeros((1,9 * 6 * 6),dtype=np.float32)
        for i in range(9 * 6):
            array[0][i * 6 + self.state[i]] = 1
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
        return int("".join([str(i) for i in self.state]),6)

    def reward(self):
        if len(REWARDS) == 0:
            print("ERROR")
        hash = self.hash()
        if hash in REWARDS:
            return REWARDS[hash]
        else:
            return 0

# Deprecating
def calculate_rewards(depth = 8):
    # Calculate the rewards
    discount = 0.8
    stack = [Environment()]

    for i in range(depth):
        print(f"Calculating rewards for depth {i}")
        t_stack = stack.copy()
        stack = []
        for item in t_stack:
            # compute hash of item
            hash = item.hash()
            if hash not in REWARDS:
                REWARDS[hash] = 1 * (discount**i)
                for action in ACTIONS:
                    env = item.copy()
                    env.apply_action(action)
                    stack.append(env)
    print(f"Calculated rewards for {len(REWARDS)} states")

# Deprecating
def create_scrambled_env(scramble_depth):
    env = Environment()
    env.scramble(scramble_depth)
    return env

# Deprecating
def action_from_choice(choice):
    return ACTIONS[choice[0]]
