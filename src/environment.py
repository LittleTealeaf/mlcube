import numpy as np


class Action:
    "Represents an action that a state of the cube can transform through"
    def __init__(self, name: str, loops: list[list[int]], two: bool = False, prime: bool = False) -> None:
        self.name = name
        "The displayable name of the action"
        self.matrix = np.identity(9 * 6, dtype=np.int8)
        "The transformation matrix that the move applies to the state"

        for loop in loops:
            initial = np.copy(self.matrix[loop[0]])

            for i in range(len(loop) - 1):
                self.matrix[loop[i]] = self.matrix[loop[-1]]
            self.matrix[loop[-1]] = initial

            del initial

        if two:
            self.matrix = self.matrix @ self.matrix

        if prime:
            self.matrix = self.matrix @ self.matrix @ self.matrix

    def apply(self, state):
        "Applies a move to the provided state"
        return state @ self.matrix


def create_moves(name: str, loop: list[list[int]]):
    "Creates a set of moves, including the provided move, the move performed twice, and the move performed three times (aka. reverse)"
    return [
        Action(name, loop),
        Action(f"{name}P", loop, prime=True),
        Action(f"{name}2", loop, two=True)
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
"A set of all possible moves for any state of the rubik's cube"


ACTION_COUNT = len(ACTIONS)
"The number of possible moves"
