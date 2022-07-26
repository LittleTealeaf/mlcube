from enum import Enum
import numpy as np

# https://encrypted-tbn0.gstatic.com/images?q=tbn:ANd9GcQhXUyatM9IZWfEoxXm2sHwbjr_6l_y-uTGXg&usqp=CAU
# https://ruwix.com/pics/rubiks-cube/mathematics-permutation-group.jpg


class Color(Enum):
    "W"
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


class Move:
    def build(moves: list[list[int]]):
        m = np.identity(9 * 6, dtype=np.int8)

        for loop in moves:
            first = np.copy(m[loop[0]])
            for i in range(len(loop) - 1):
                m[loop[i]] = m[loop[i + 1]]
            m[loop[-1]] = first

        return m
    def two(moves) -> np.ndarray[(54,54),np.int8]:
        return moves @ moves
    def prime(moves) -> np.ndarray[(54,54),np.int8]:
        return moves.T


    R = build(
        [
            [20, 2, 42, 47],
            [23, 5, 39, 50],
            [26, 8, 36, 53],
            [27, 29, 35, 33],
            [28, 32, 34, 30],
        ]
    )
    R2 = two(R)
    RP = prime(R)
    U = build(
        [
            [20, 11, 38, 29],
            [19, 10, 37, 28],
            [18, 9, 36, 27],
            [8, 6, 0, 2],
            [7, 3, 1, 5],
        ]
    )
    U2 = two(U)
    UP = prime(U)
    L = build(
        [
            [18, 45, 44, 0],
            [21, 48, 41, 3],
            [24, 51, 38, 6],
            [11, 17, 15, 9],
            [14, 16, 12, 10],
        ]
    )
    L2 = two(L)
    LP = prime(L)
    D = build(
        [
            [24, 33, 42, 15],
            [25, 34, 43, 16],
            [26, 35, 44, 17],
            [45, 47, 53, 51],
            [46, 50, 52, 48],
        ]
    )
    D2 = two(D)
    DP = prime(D)
    F = build(
        [
            [6, 27, 47, 17],
            [7, 30, 46, 14],
            [8, 33, 45, 11],
            [18, 20, 26, 24],
            [19, 23, 25, 21],
        ]
    )
    FP = prime(F)
    F2 = two(F)
    B = build(
        [
            [36, 38, 44, 42],
            [37, 41, 43, 39],
            [29, 0, 15, 53],
            [32, 1, 12, 52],
            [35, 2, 9, 51],
        ]
    )
    BP = prime(B)
    B2 = two(B)


print(Move.R)


class Cube:
    def __init__(self):
        self.state = np.zeros((9 * 6), dtype=np.int8)
        for i in range(self.state.size):
            self.state[i] = i / 9



c = Cube()
c.state = c.state @ Move.R @ Move.R @ Move.L @ Move.L @ Move.U @ Move.U @ Move.D @ Move.D @ Move.F @ Move.F @ Move.B @ Move.B
print(c.state)
