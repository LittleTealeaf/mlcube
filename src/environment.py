import numpy as np
from random import Random


class Action:
    """
    A class representing an action performed on the cube

    Attributes
    ----------
    name: str
        The displayable name representing the action on the cube
    matrix: NDArray[int8]
        The transformation matrix that is applied to a state when the action is performed

    Methods
    -------
    apply(environment)
        Applies the transformation matrix to the environment, returning a new environment state
    """

    def __init__(self, name: str, loops: list[list[int]], two: bool = False, prime: bool = False) -> None:
        """
        Creates an Action.

        Parameters
        ----------
        name: str
            The name of the Action. This is used when printing out solutions, or serializing move sequences

        loops: list[list[int]]
            The loop to use for the base move. This is structured as a list of individual loops within the move. The loops should track the movement of each individual tile on the cube during that move.

        two: bool = False
            If true, indicates that the action should be transformed to the 2x version, or the resulting action if the provided base action was taken twice

        prime: bool = False
            If true, indicates that the action should be transformed to the prime version, or the resulting action if the provided base action was taken three times.

        """
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


def create_actions(name: str, loop: list[list[int]]) -> list[Action]:
    """
    Creates a set of actions from a base name and its loop.

    Parameters
    ----------
    name: str
        The base name of the action. For example, for a right-side move, this will be "R". This is then used to build the x2 move ("R2") and the reverse move ("RP")

    loop: list[list[int]]
        The loop to use for the base action. This is structured as a list of individual loops within the move. The loops should track the movement of each individual tile on the cube during that action.

    Returns
    -------
    list[Action, Action, Action]
        Returns a list of actions. The first action is the core action, the second action is the action's 2x version, and the last action is that action's reverse action.

    """
    return [
        Action(name, loop),
        Action(f"{name}P", loop, prime=True),
        Action(f"{name}2", loop, two=True)
    ]


ACTIONS = [
    move
    for moves in [
        create_actions(
            "R",
            [
                [20, 2, 42, 47],
                [23, 5, 39, 50],
                [26, 8, 36, 53],
                [27, 29, 35, 33],
                [28, 32, 34, 30],
            ],
        ),
        create_actions(
            "U",
            [
                [20, 11, 38, 29],
                [19, 10, 37, 28],
                [18, 9, 36, 27],
                [8, 6, 0, 2],
                [7, 3, 1, 5],
            ],
        ),
        create_actions(
            "L",
            [
                [18, 45, 44, 0],
                [21, 48, 41, 3],
                [24, 51, 38, 6],
                [11, 17, 15, 9],
                [14, 16, 12, 10],
            ],
        ),
        create_actions(
            "D",
            [
                [24, 33, 42, 15],
                [25, 34, 43, 16],
                [26, 35, 44, 17],
                [45, 47, 53, 51],
                [46, 50, 52, 48],
            ],
        ),
        create_actions(
            "F",
            [
                [6, 27, 47, 17],
                [7, 30, 46, 14],
                [8, 33, 45, 11],
                [18, 20, 26, 24],
                [19, 23, 25, 21],
            ],
        ),
        create_actions(
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


def create_environment(scramble_depth: int = 0, random=Random()):
    """
    Creates a new environment with optional scrambling

    Parameters
    ----------
    scrabmle_depth: int = 0
        The depth that the cube should be scrambled to. The moves used to scramble the cube is randomized using the random function. (default is 0, which returns a cube in the solved state)       

    random: Random = Random()
        The object used to randomly choose scrambling moves. This allows making identical scrambled instances by making multiple environments using the same seeded random object. (default to a new Random instance)

    Returns
    -------
    environment
        A new environment created using the specified parameters. This will be in the form of a numpy array

    """
    # Creates an empty environment numpy array, initializing the colors using the lambda function
    # The lambda function indicates that the colors should be incrementally set every 9 values 
    env: np.ndarray = np.fromfunction(lambda i: i // 9, (9*6,))
    # Scrambles the cube using random.choice() to apply to the environment
    for _ in range(scramble_depth):
        env = random.choice(ACTIONS).apply(env)

    return env

