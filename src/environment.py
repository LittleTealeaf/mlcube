import numpy as np


class Action:
    "Represents an action that can performed on a cube"

    def __init__(self, name: str, permutations: list[list[int]], two=False, prime=False):
        self._name = name
        self._permutations: list[list[int]] = []

        for perm in permutations:
            if prime:
                perm.reverse()
                self._permutations.append(perm)
            elif two:
                # Split up perm into two
                a = []
                b = []
                for i in range(len(perm)):
                    if i % 2 == 0:
                        a.append(perm[i])
                    else:
                        b.append(perm[i])
                self._permutations.append(a)
                self._permutations.append(b)
            else:
                self._permutations.append(perm)


def create_moves(name: str, permutations: list[list[int]]) -> list[Action]:
    return [
        Action(name, permutations),
        Action(f"{name}P", permutations, prime=True)
    ]


ACTION_SETS = [
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

ACTIONS = [
    move
    for moves in ACTION_SETS
    for move in moves
]


class Cube:
    def __init__(self):
        self._state = np.zeros((54,), dtype=np.int8)
        for i in range(54):
            self._state[i] = i // 9

    def apply(self, action: Action):
        for perm in action._permutations:
            last = self._state[perm[-1]]
            for i in range(1, len(perm)):
                self._state[perm[i]] = self._state[perm[i-1]]

            self._state[perm[0]] = last
