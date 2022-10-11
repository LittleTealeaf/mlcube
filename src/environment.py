from random import Random
import numpy as np
import tensorflow as tf

class Action:
    def __init__(self, name: str, loops: list[list[int]], two=False, prime=False):
        self.name = name
        matrix = np.identity(9 * 6,dtype=np.float32)

        for loop in loops:
          initial = np.copy(matrix[loop[0]])
          for i in range(len(loop) - 1):
            matrix[loop[i]] = matrix[loop[i+1]]
          matrix[loop[-1]] = initial

        del initial

        if two:
            matrix = matrix @ matrix

        if prime:
            matrix = matrix @ matrix @ matrix

        self.matrix = np.zeros((9*6*6,9*6*6))

        for x in range(9 * 6):
            for y in range(9 * 6):
                if matrix[y,x] == 1:
                    for i in range(6):
                        self.matrix[y * 6 + i, x * 6 + i] = 1


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


# class Environment:
#     def __init__(self, observations: list[list[int]] = None):
#         if observations:
#             self.state = np.array([0] * (9 * 6),dtype=np.int8)
#             for i in range(len(observations[0])):
#                 self.state[i//6] += observations[0][i] * (i%6)
#         else:
#             self.reset()


#     def reset(self):
#         self.state = np.array([i // 9 for i in range(9 * 6)],dtype=np.int8)
#         return self

#     def apply_action(self,action: Action):
#       self.state = action.apply(self.state)
#       return self

#     def is_complete(self):
#       for i in range(9 * 6):
#         if self.state[i] != i // 9:
#           return False
#       return True

#     def to_observations(self):
#         array = np.zeros((1,9 * 6 * 6),dtype=np.float32)
#         for i in range(9 * 6):
#             array[0][i * 6 + self.state[i]] = 1
#         return array

#     def scramble(self,count: int = 100):
#         random = Random()
#         for _ in range(count):
#             self.apply_action(random.choice(ACTIONS))
#         return self

#     def copy(self):
#         env = Environment()
#         env.state = np.copy(self.state)
#         return env

#     def hash(self):
#         return int("".join([str(i) for i in self.state]),6)

#     def reward(self, rewards):
#         if len(rewards) == 0:
#             print("ERROR")
#         hash = self.hash()
#         try:
#             return rewards[hash]
#         except KeyError:
#             return 0

# class Environment:
#     def __init__(self,state = None):
#         if state:
#             self.state = state
#         else:
#             # So I'm either going crazy and this doesn't work, or I'm going crazy and this does work
#             self.state = np.array([
#                 1
#                 if (i // 6) == i % 6
#                 else 0
#                 for i in range(9*6*6)
#             ])

#     def getTensor(self,overwrite=False):
#         if not self.tensor or overwrite:
#             self.tensor = tf.constant(self.state)

#         return self.tensor

#     def getState(self,overwrite=False):
#         if overwrite and self.tensor:
#             self.state = self.getTensor(False)
#         return self.state

#     def hash(self):

def create_environment(scramble_length=0):
    # variable = tf.constant(np.array([
    #     1 if i // (6*9) == i % 6 else 0
    #     for i in range(9*6*6)
    # ]),dtype=tf.float32)
    # random = Random()
    # for i in range(scramble_length):
    #     variable = tf.matmul(variable,tf.constant)
    # return variable
    state = np.array([
        1 if i // (6 * 9) == i % 6 else 0
        for i in range(9*6*6)
    ])
    random = Random()
    for _ in range(scramble_length):
        state = state @ random.choice(ACTIONS).matrix
    return tf.constant(state,dtype=tf.float32)


@tf.function
def hash_environment(env):
    tf_string_tensor = tf.strings.as_string(env)
    tf_string_concatenated = tf.strings.join(tf_string_tensor)
    return tf_string_concatenated
    # TODO hash a sparse array


ACTIONS_TENSOR = tf.constant(np.array([
    action.matrix for action in ACTIONS
]),dtype=tf.float32)



# def calculate_rewards(depth=8,decay=0.8,max_count=1_000_000):
#     rewards = {}
#     count = 0
#     buffer = [Environment()]
#     for i in range(depth):
#         print(f"Calculating depth {i} with length {len(buffer)}")
#         tmp_buffer = []
#         for env in buffer:
#             hash = env.hash()
#             if hash not in rewards:
#                 rewards[hash] = decay ** i
#                 count = count + 1
#                 if count >= max_count:
#                     print("Hit maximum reward length")
#                     return rewards

#                 if i < depth - 1:
#                     for action in ACTIONS:
#                         tmp_buffer.append(env.copy().apply_action(action))
#         buffer = tmp_buffer
#     return rewards

# def create_scrambled_environment(depth):
#     env = Environment()
#     env.scramble(depth)
#     return env

ACTION_COUNT = len(ACTIONS)
