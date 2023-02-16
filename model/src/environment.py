import numpy as np

class Action:
    def __init__(self, name, loops: list[list[int]], two=False, prime=False):
        self._name = name

        if prime:
            loops = [loop[::-1] for loop in loops]
        if two:
            loops = [
                item
                for items in [[loop[::2], loop[1::2]] for loop in loops]
                for item in items
            ]



        self._permutations = loops


def create_moves(name: str, loops: list[list[int]]):
    return [
        Action(name, loops),
        Action(f"{name}2", loops, two=True),
        Action(f"{name}P", loops, prime=True),
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



class Environment:
    def __init__(self):
        ...


# # %%

# This means that we can use array slicing with slicing arrays to automate things! :O
# import numpy as np

# arr = np.array([1,2,3,4])
# print(arr)
# arr[[2,3]] = arr[[3,2]]
# print(arr)

# # %%


if __name__=='__main__':
    for action in ACTIONS:
        print(action._name, action._permutations)
