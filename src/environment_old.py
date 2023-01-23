import numpy as np
from numpy._typing import NDArray
import tensorflow as tf
from tensorflow.python.framework.dtypes import Type
from tf_agents.trajectories import time_step
from tf_agents.environments.py_environment import PyEnvironment
from tf_agents.specs.array_spec import BoundedArraySpec
from tf_agents.trajectories import TimeStep
from tf_agents.typing.types import NestedArray, NestedArraySpec
from random import Random


class Action:
    def __init__(self, name: str, loops: list[list[int]], two=False, prime=False):
        self._name = name
        self._matrix: NDArray[np.int8] = np.identity(9*6, dtype=np.int8)

        for loop in loops:
            initial = np.copy(self._matrix[loop[0]])
            for i in range(len(loop) - 1):
                self._matrix[loop[i]] = self._matrix[loop[i+1]]
            self._matrix[loop[-1]] = initial

        if two:
            self._matrix: NDArray[np.int8] = self._matrix @ self._matrix

        if prime:
            self._matrix: NDArray[np.int8] = self._matrix @ self._matrix @ self._matrix

    def apply(self, state: NDArray[np.int8]) -> NDArray[np.int8]:
        return state @ self._matrix


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


class CubeEnvironment(PyEnvironment):
    def __init__(self, seed: float | None = None, moves_max: int = 500, print_steps=False):

        self._seed: float | None = seed
        self._moves_max = moves_max
        self._moves = 0
        self._print_steps = print_steps

        self._action_spec = BoundedArraySpec(
            shape=(), dtype=np.int32, minimum=0, maximum=18, name='action')
        """The specs of the actions space. States that there is one action choice, and it has a minimum of 0 and maximum of 18. If the system returns 18, then it indicates that it is done and ready to evaluate"""
        self._observation_spec = BoundedArraySpec(
            shape=(9 * 6 * 6,), dtype=np.int32, name='observation', minimum=np.array([0] * 9 * 6 * 6), maximum=[1] * 9 * 6 * 6)
        """The specs of the observation space. States that there is an array of 9 * 6 * 6 entries, each one between 0 and 1 (either 0 or 1)"""
        self._state: NDArray[np.int8] = np.fromfunction(
            lambda i: i // 9, (56,))
        """The state of the cube. It's basically a numpy array of 56 values"""
        self._episode_ended = False
        """Whether the episode has already ended"""

    def action_spec(self) -> NestedArraySpec:
        return self._action_spec

    def observation_spec(self) -> NestedArraySpec:
        return self._observation_spec

    def _reset(self) -> TimeStep:
        self._state = np.fromfunction(lambda i: i // 9, (9*6,))
        self._moves = 0
        self._step_moves = []

        random = Random()
        for _ in range(100):
            self._state = random.choice(ACTIONS).apply(self._state)
        self._episode_ended = False
        return time_step.restart(self.get_observations())

    def get_observations(self):
        obs = np.zeros((9*6*6,), dtype=np.int32)
        for i in range(9*6):
            index = int(self._state[i])
            obs[i * 6 + index] = 1
        return obs

    def get_reward(self):
        reward_total = 0.0
        for i in range(9*6):
            if self._state[i] == i // 9:
                reward_total += 1
        return reward_total

    def is_solved(self) -> bool:
        for i in range(9*6):
            if self._state[i] != i // 9:
                return False
        return True

    def _step(self, action) -> TimeStep:

        if self._episode_ended:
            return self.reset()

        if action == 18 or self._moves > self._moves_max:
            if action == 18:
                self._step_moves.append("Stop")
            else:
                self._step_moves.append("...")
            self._episode_ended = True
        else:
            try:
                self._state = ACTIONS[action].apply(self._state)
                self._step_moves.append(ACTIONS[action]._name)
            except:
                self._state = ACTIONS[action[0]].apply(self._state)
                self._step_moves.append(ACTIONS[action[0]]._name)

        observations = self.get_observations()

        self._moves += 1 

        reward = self.get_reward()
        if self._episode_ended:
            if self._print_steps:
                print("moves: ",",".join(self._step_moves))
            return time_step.termination(observations, reward=reward)
        else:
            return time_step.transition(observations, reward=0.0)

        
