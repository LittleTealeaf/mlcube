import numpy as np
import tensorflow as tf
from numpy._typing import NDArray
from tf_agents.environments import TFEnvironment


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

        self._tensor = tf.constant(self._matrix)

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


class CubeEnvironment(TFEnvironment):
    def __init__(self, seed: float | None = None, moves_max: int = 500, print_steps: bool = False, batch_size: int = 1):

        

        super(CubeEnvironment, self).__init__()
