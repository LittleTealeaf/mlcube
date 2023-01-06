from __future__ import absolute_import
from __future__ import division
from __future__ import print_function

import abc
import tensorflow as tf
import numpy as np
from random import Random

from tf_agents.environments import py_environment
from tf_agents.environments import tf_environment
from tf_agents.environments import tf_py_environment
from tf_agents.environments import utils
from tf_agents.specs import array_spec
from tf_agents.environments import wrappers
from tf_agents.environments import suite_gym
from tf_agents.trajectories import time_step as ts


class Action:
    def __init__(self, name: str, loops: list[list[int]], two=False, prime=False):
        self.name = name
        self.matrix = np.identity(9 * 6, dtype=np.int8)

        for loop in loops:
            initial = np.copy(self.matrix[loop[0]])
            for i in range(len(loop) - 1):
                self.matrix[loop[i]] = self.matrix[loop[i+1]]
            self.matrix[loop[-1]] = initial

        if two:
            self.matrix = self.matrix @ self.matrix

        if prime:
            self.matrix = self.matrix @ self.matrix @ self.matrix

    def apply(self, state):
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


class RubiksCubeEnvironment(py_environment.PyEnvironment):
    def __init__(self, episode_max_moves=1000):
        self._action_spec = array_spec.BoundedArraySpec(
            shape=(), dtype=np.int32, minimum=0, maximum=len(ACTIONS) - 1, name='action'
        )
        self._observation_spec = array_spec.BoundedArraySpec(
            shape=(9*6*6,), dtype=np.int32, minimum=0, maximum=1, name='observations'
        )
        self._state = np.fromfunction(lambda i: i // 9, (9*6,))
        self._moves = 0
        self._episode_ended = False
        self._episode_max_moves = episode_max_moves

    def action_spec(self):
        return self._action_spec

    def observation_spec(self):
        return self._observation_spec

    def is_solved(self):
        for i in range(9 * 6):
            if self._state[i] != i // 9:
                return False
        return True

    def get_observations(self):
        obs = np.zeros((9*6*6), dtype=np.int32)
        for i in range(9*6):
            index = int(self._state[i])
            obs[i * 6 + index] = 1
        return obs

    def get_reward(self):
        reward_total = 0.0
        for i in range(9*6):
            if self._state[i] == i // 9:
                reward_total += 1
        return reward_total / (9 * 6)

    def _reset(self):
        # TODO add scrambled moves to reset
        self._moves = 0
        self._state = np.fromfunction(lambda i: i // 9, (9*6,))
        random = Random()
        for _ in range(100):
            self._state = random.choice(ACTIONS).apply(self._state)
        self._episode_ended = False
        return ts.restart(self.get_observations())

    def _step(self, action):
        self._moves += 1
        self._state = ACTIONS[int(action)].apply(self._state)

        if self.is_solved():
            self._episode_ended = True

        observations = self.get_observations()

        if self._episode_ended or self._moves >= self._episode_max_moves:
            # calculate reward

            reward = self.get_reward()
            return ts.termination(observations, reward)
        else:
            return ts.transition(observations, reward=0.0, discount=1.0)
