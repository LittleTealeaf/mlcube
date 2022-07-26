from enum import Enum
import numpy as np


class Color(Enum):
    W = 0
    WHITE = 0
    Y = 1
    YELLOW = 1
    G = 2
    GREEN = 2
    B = 3
    BLUE = 3
    R = 4
    RED = 4
    O = 5
    ORGANE = 5


# https://upload.wikimedia.org/wikipedia/commons/thumb/1/1d/Japanese_color_scheme_of_a_Rubik%27s_Cube.svg/1200px-Japanese_color_scheme_of_a_Rubik%27s_Cube.svg.png


class Cube:
    def __init__(self):
        self.state = np.matrix(
            [
                [-1, -1, -1, 0, 0, 0, -1, -1, -1, -1, -1, -1],
                [-1, -1, -1, 0, 0, 0, -1, -1, -1, -1, -1, -1],
                [-1, -1, -1, 0, 0, 0, -1, -1, -1, -1, -1, -1],
                [5, 5, 5, 2, 2, 2, 4, 4, 4, 3, 3, 3],
                [5, 5, 5, 2, 2, 2, 4, 4, 4, 3, 3, 3],
                [5, 5, 5, 2, 2, 2, 4, 4, 4, 3, 3, 3],
                [-1, -1, -1, 1, 1, 1, -1, -1, -1, -1, -1, -1],
                [-1, -1, -1, 1, 1, 1, -1, -1, -1, -1, -1, -1],
                [-1, -1, -1, 1, 1, 1, -1, -1, -1, -1, -1, -1],
            ]
        )
        print(self.state.shape)

    def R(self):
        self.state[0, 5], self.state[5, 9], self.state[6, 5], self.state[3, 5] = (
            self.state[3, 5],
            self.state[0, 5],
            self.state[5, 9],
            self.state[6, 5],
        )
        self.state[1, 5], self.state[4, 9], self.state[7, 5], self.state[4, 5] = (
            self.state[4, 5],
            self.state[1, 5],
            self.state[4, 9],
            self.state[7, 5],
        )
        self.state[2, 5], self.state[3, 9], self.state[8, 5], self.state[5, 5] = (
            self.state[5, 5],
            self.state[2, 5],
            self.state[3, 9],
            self.state[8, 5],
        )
        self.state[3, 6], self.state[3, 8], self.state[5, 8], self.state[5, 6] = (
            self.state[5, 6],
            self.state[3, 6],
            self.state[3, 8],
            self.state[5, 8],
        )
        self.state[3, 7], self.state[4, 8], self.state[5, 7], self.state[4, 6] = (
            self.state[4, 6],
            self.state[3, 7],
            self.state[4, 8],
            self.state[5, 7],
        )

    def RP(self):
        self.state[0, 5], self.state[5, 9], self.state[6, 5], self.state[3, 5] = (
            self.state[5, 9],
            self.state[6, 5],
            self.state[3, 5],
            self.state[0, 5],
        )
        self.state[1, 5], self.state[4, 9], self.state[7, 5], self.state[4, 5] = (
            self.state[4, 9],
            self.state[7, 5],
            self.state[4, 5],
            self.state[1, 5],
        )
        self.state[2, 5], self.state[3, 9], self.state[8, 5], self.state[5, 5] = (
            self.state[3, 9],
            self.state[8, 5],
            self.state[5, 5],
            self.state[2, 5],
        )
        self.state[3, 6], self.state[3, 8], self.state[5, 8], self.state[5, 6] = (
            self.state[3, 8],
            self.state[5, 8],
            self.state[5, 6],
            self.state[3, 6],
        )
        self.state[3, 7], self.state[4, 8], self.state[5, 7], self.state[4, 6] = (
            self.state[4, 8],
            self.state[5, 7],
            self.state[4, 6],
            self.state[3, 7],
        )

    def L(self):
        self.state[0, 3], self.state[3, 3], self.state[6, 3], self.state[5, 11] = (
            self.state[5, 11],
            self.state[0, 3],
            self.state[3, 3],
            self.state[6, 3],
        )
        self.state[1, 3], self.state[4, 3], self.state[7, 3], self.state[4, 11] = (
            self.state[4, 11],
            self.state[1, 3],
            self.state[4, 3],
            self.state[7, 3],
        )
        self.state[2, 3], self.state[5, 3], self.state[8, 3], self.state[3, 11] = (
            self.state[3, 11],
            self.state[2, 3],
            self.state[5, 3],
            self.state[8, 3],
        )
        self.state[3, 0], self.state[3, 2], self.state[5, 2], self.state[5, 0] = (
            self.state[5, 0],
            self.state[3, 0],
            self.state[3, 2],
            self.state[5, 2],
        )
        self.state[4, 0], self.state[3, 1], self.state[4, 2], self.state[5, 1] = (
            self.state[5, 1],
            self.state[4, 0],
            self.state[3, 1],
            self.state[4, 2],
        )

    def LP(self):
        self.state[0, 3], self.state[3, 3], self.state[6, 3], self.state[5, 11] = (
            self.state[3, 3],
            self.state[6, 3],
            self.state[5, 11],
            self.state[0, 3],
        )
        self.state[1, 3], self.state[4, 3], self.state[7, 3], self.state[4, 11] = (
            self.state[4, 3],
            self.state[7, 3],
            self.state[4, 11],
            self.state[1, 3],
        )
        self.state[2, 3], self.state[5, 3], self.state[8, 3], self.state[3, 11] = (
            self.state[5, 3],
            self.state[8, 3],
            self.state[3, 11],
            self.state[2, 3],
        )
        self.state[3, 0], self.state[3, 2], self.state[5, 2], self.state[5, 0] = (
            self.state[3, 2],
            self.state[5, 2],
            self.state[5, 0],
            self.state[3, 0],
        )
        self.state[4, 0], self.state[3, 1], self.state[4, 2], self.state[5, 1] = (
            self.state[3, 1],
            self.state[4, 2],
            self.state[5, 1],
            self.state[4, 0],
        )

    def U(self):
        self.state[3, 3], self.state[3, 0], self.state[3, 9], self.state[3, 6] = (
            self.state[3, 6],
            self.state[3, 3],
            self.state[3, 0],
            self.state[3, 9],
        )
        self.state[3, 4], self.state[3, 1], self.state[3, 10], self.state[3, 7] = (
            self.state[3, 7],
            self.state[3, 4],
            self.state[3, 1],
            self.state[3, 10],
        )
        self.state[3, 5], self.state[3, 2], self.state[3, 11], self.state[3, 8] = (
            self.state[3, 8],
            self.state[3, 5],
            self.state[3, 2],
            self.state[3, 11],
        )
        self.state[0, 3], self.state[0, 5], self.state[2, 5], self.state[2, 3] = (
            self.state[2, 3],
            self.state[0, 3],
            self.state[0, 5],
            self.state[2, 5],
        )
        self.state[0, 4], self.state[1, 5], self.state[2, 4], self.state[1, 3] = (
            self.state[1, 3],
            self.state[0, 4],
            self.state[1, 5],
            self.state[2, 4],
        )

    def UP(self):
        self.state[3, 3], self.state[3, 0], self.state[3, 9], self.state[3, 6] = (
            self.state[3, 0],
            self.state[3, 9],
            self.state[3, 6],
            self.state[3, 3],
        )
        self.state[3, 4], self.state[3, 1], self.state[3, 10], self.state[3, 7] = (
            self.state[3, 1],
            self.state[3, 10],
            self.state[3, 7],
            self.state[3, 4],
        )
        self.state[3, 5], self.state[3, 2], self.state[3, 11], self.state[3, 8] = (
            self.state[3, 2],
            self.state[3, 11],
            self.state[3, 8],
            self.state[3, 5],
        )
        self.state[0, 3], self.state[0, 5], self.state[2, 5], self.state[2, 3] = (
            self.state[0, 5],
            self.state[2, 5],
            self.state[2, 3],
            self.state[0, 3],
        )
        self.state[0, 4], self.state[1, 5], self.state[2, 4], self.state[1, 3] = (
            self.state[1, 5],
            self.state[2, 4],
            self.state[1, 3],
            self.state[0, 4],
        )

    def D(self):
        self.state[5, 3], self.state[5, 6], self.state[5, 9], self.state[5, 0] = (
            self.state[5, 0],
            self.state[5, 3],
            self.state[5, 6],
            self.state[5, 9],
        )
        self.state[5, 4], self.state[5, 7], self.state[5, 10], self.state[5, 1] = (
            self.state[5, 1],
            self.state[5, 4],
            self.state[5, 7],
            self.state[5, 10],
        )
        self.state[5, 5], self.state[5, 8], self.state[5, 11], self.state[5, 2] = (
            self.state[5, 2],
            self.state[5, 5],
            self.state[5, 8],
            self.state[5, 11],
        )
        self.state[6, 3], self.state[6, 5], self.state[8, 5], self.state[8, 3] = (
            self.state[8, 3],
            self.state[6, 3],
            self.state[6, 5],
            self.state[8, 5],
        )
        self.state[6, 4], self.state[7, 5], self.state[8, 4], self.state[8, 3] = (
            self.state[8, 3],
            self.state[6, 4],
            self.state[7, 5],
            self.state[8, 4],
        )

    def DP(self):
        self.state[5, 3], self.state[5, 6], self.state[5, 9], self.state[5, 0] = (
            self.state[5, 6],
            self.state[5, 9],
            self.state[5, 0],
            self.state[5, 3],
        )
        self.state[5, 4], self.state[5, 7], self.state[5, 10], self.state[5, 1] = (
            self.state[5, 7],
            self.state[5, 10],
            self.state[5, 1],
            self.state[5, 4],
        )
        self.state[5, 5], self.state[5, 8], self.state[5, 11], self.state[5, 2] = (
            self.state[5, 8],
            self.state[5, 11],
            self.state[5, 2],
            self.state[5, 5],
        )
        self.state[6, 3], self.state[6, 5], self.state[8, 5], self.state[8, 3] = (
            self.state[6, 5],
            self.state[8, 5],
            self.state[8, 3],
            self.state[6, 3],
        )
        self.state[6, 4], self.state[7, 5], self.state[8, 4], self.state[8, 3] = (
            self.state[7, 5],
            self.state[8, 4],
            self.state[8, 3],
            self.state[6, 4],
        )


# it's ~15 times slower than java

print(Cube().state)
