import numpy as np
import tensorflow as tf
from random import Random
from functools import reduce
from multiprocessing import Pool
import pandas as pd
import os
import json


class Move:
    def __init__(
        self, name: str, loops: list[list[int]], two: bool = False, prime: bool = False
    ):
        self.name = name
        self.matrix: np.ndarray[(54,54),np.int8] = np.identity(9 * 6, dtype=np.int8)
        for loop in loops:
            first = np.copy(self.matrix[loop[0]])
            for i in range(len(loop) - 1):
                self.matrix[loop[i]] = self.matrix[loop[i + 1]]
            self.matrix[loop[-1]] = first
        if two:
            self.matrix = self.matrix @ self.matrix
        if prime:
            self.matrix = self.matrix.T

    def apply(self,state: np.ndarray[(54),np.float32]):
        return state @ self.matrix


def create_move(letter: str, loops: list[list[int]]) -> list[Move]:
    "Creates a move, it's prime syntax, and it's double syntax"
    return [
        Move(letter, loops),
        Move(f"{letter}P", loops, prime=True),
        Move(f"{letter}2", loops, two=True),
    ]


# Behold, python syntax
MOVES = [
    move
    for moves in [
        create_move(
            "R",
            [
                [20, 2, 42, 47],
                [23, 5, 39, 50],
                [26, 8, 36, 53],
                [27, 29, 35, 33],
                [28, 32, 34, 30],
            ],
        ),
        create_move(
            "U",
            [
                [20, 11, 38, 29],
                [19, 10, 37, 28],
                [18, 9, 36, 27],
                [8, 6, 0, 2],
                [7, 3, 1, 5],
            ],
        ),
        create_move(
            "L",
            [
                [18, 45, 44, 0],
                [21, 48, 41, 3],
                [24, 51, 38, 6],
                [11, 17, 15, 9],
                [14, 16, 12, 10],
            ],
        ),
        create_move(
            "D",
            [
                [24, 33, 42, 15],
                [25, 34, 43, 16],
                [26, 35, 44, 17],
                [45, 47, 53, 51],
                [46, 50, 52, 48],
            ],
        ),
        create_move(
            "F",
            [
                [6, 27, 47, 17],
                [7, 30, 46, 14],
                [8, 33, 45, 11],
                [18, 20, 26, 24],
                [19, 23, 25, 21],
            ],
        ),
        create_move(
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


def create_cube():
    state: np.ndarray[(54),np.int8] = np.zeros((54),dtype=np.int8)
    for i in range(54):
        state[i] = i // 9
    return state

def scramble_cube(cube: np.ndarray[(54),np.int8],count: int = 100):
    random = Random()
    return cube @ reduce(
        lambda a, b: a @ b, [random.choice(MOVES).matrix for i in range(count)]
    )

def create_scrambled_cube(scramble_length: int):
    return scramble_cube(create_cube(),scramble_length)

def create_scrambled_sample(count: int):
    with Pool() as pool:
        return pool.map(create_scrambled_cube,[100] * count)

if __name__ == '__main__':
    print("start")
    create_scrambled_sample(1000)
    print("done")
