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
    def get_reward(self) -> float:
        "Returns the calculated reward of the cube at the current state"
        ...
    def get_name(self) -> str:
        "Returns the name of the cube"
        ...


class PyCube2x2(Puzzle):
    ...

class PyCube3x3(Puzzle):
    ...

class Replay(Puzzle):
    def __init__(self, capacity: int = 100_000):
        "Creates a replay with the specified capacity"
        ...
    def sample_replay(self, count: int) -> list[tuple[list[int], int, float, list[int]]]:
        ...
    def is_at_capacity(self) -> bool:
        ...

class PyReplay2x2(Replay):
    ...

class PyReplay3x3(Replay):
    ...
