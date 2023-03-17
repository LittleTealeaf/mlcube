class Puzzle:
    observation_length: int
    action_size: int
    def __init__(self):
        ...
    def reset(self):
        ...
    def is_solved(self) -> bool:
        ...
    def get_observations(self) -> list[int]:
        ...
    def apply_action(self, action: int):
        ...
    def scramble(self, steps: int) -> int:
        ...
    def scramble_with_seed(self, steps: int, seed: int):
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
