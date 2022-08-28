import numpy as np
import tensorflow as tf

class Move:
    def __init__(
        self, name: str, loops: list[list[int]], two: bool = False, prime: bool = False
    ):
        self.name = name
        copy = np.identity(9 * 6 * 6, dtype=np.float32)
        matrix = np.copy(copy)

        for offset in range(6):
            for loop in loops:
                for i in range(len(loop) - 1):
                    matrix[loop[i] + offset] = copy[loop[i+1] + offset]
                matrix[loop[-1] + offset] = copy[loop[0] + offset]

        if two:
            matrix = matrix @ matrix
        if prime:
            matrix = matrix.T

        self.tensor = tf.constant(matrix,dtype=tf.float32,name=f'MOVE{self.name}')

    def apply(self, state: np.ndarray[54, np.float32]):
        return tf.matmul(state,self.tensor)


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
