class Puzzle:
    observation_length: int
    "The length of array that the `get_observations()` method will return. That is, any state output of this puzzle will have a length of this value"
    action_size: int
    "The number of valid actions that can be performed on the puzzle. Determines the maximum value that can be used in the `apply_action()` method"
    def __init__(self):
        "Creates a new instance of the puzzle in its solved state"
        ...
    def reset(self):
        "Resets the puzzle to its solved state"
        ...
    def is_solved(self) -> bool:
        "Returns `True` if the puzzle is in its solved state, otherwise returns `False`"
        ...
    def get_observations(self) -> list[int]:
        "Returns an array of observations generated. Each value will either be a 0 or a 1, and the length will be as defined in `observation_length`"
        ...
    def apply_action(self, action: int):
        "Applies an action to the puzzle, valid action indices are within the range `0 <= {action} < action_size`"
        ...
    def scramble(self, steps: int) -> int:
        "Scrambles the cube randomly for `steps` steps, and returns the seed used to scramble the cube"
        ...
    def scramble_with_seed(self, steps: int, seed: int):
        "Scrambles the cube randomly for `steps` steps using the provided seed."
        ...


class PyCube2x2(Puzzle):
    ...

class PyCube3x3(Puzzle):
    ...


# class PyCube2x2:
#     "Represents a 2x2x2 Rubik's Cube, commonly referred to as a '2x2 Rubik's Cube'."
#     observation_length: int
#     "The length of the array returned from `get_observations()`"
#     action_size: int
#     "The number of actions that can be applied to the cube"
#     def __init__(self) -> None:
#         "Creates a 2x2 Rubiks' Cube in its solved state"
#         ...
#     def reset(self) -> None:
#         "Resets the cube to its solved state"
#     def is_solved(self) -> bool:
#         "Returns `True` if the cube is solved, and `False` if the cube is unsolved"
#     def get_observations(self) -> list[int]:
#         """
#         Returns an observation array for the cube's current state

#         The returned array will be of length `observation_length`, and contains either `0`s or `1`s
#         """
#     def apply_action(self, action: int) -> None:
#         """
#         Applies an action to the cube.

#         The action provided must be within the range 0 and `action_size`
#         """
#     def scramble(self, steps: int) -> int:
#         """
#         Scrambles the cube by applying `n` random moves

#         Returns the seed used to scramble the cube
#         """
#     def scramble_with_seed(self, seed: int, steps: int) -> None:
#         """
#         Scrambles the cube by applying `n` random moves using the provided seed
#         """

# class PyCube3x3:
#     observation_length: int
#     action_size: int
#     def __init__(self) -> None: ...
#     def reset(self) -> None: ...
#     def is_solved(self) -> bool: ...
#     def get_observations(self) -> list[int]: ...
#     def apply_action(self, action: int) -> None: ...
#     def scramble(self, action: int) -> int: ...
#     def scramble_with_seed(self, seed: int, steps: int) -> None: ...
