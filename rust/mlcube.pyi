class PyCube2x2:
    observation_length: int
    "The length of the array returned from get_observations()"
    action_size: int
    "The number of valid actions that this puzzle allows"
    def __init__(self) -> None: ...
    "Creates a 2x2 Rubik's Cube"
    def reset(self) -> None:
        "Resets the cube to a solved state"
    def is_solved(self) -> bool:
        "Returns `True` if the cube is solved, and `False` otherwise"
    def get_observations(self) -> list[int]: ...
    def apply_action(self, action: int) -> None: ...
    def scramble(self, action: int) -> int: ...
    def scramble_with_seed(self, seed: int, steps: int) -> None: ...

class PyCube3x3:
    observation_length: int
    "The length of the array returned from get_observations()"
    action_size: int
    "The number of valid actions that this puzzle allows"
    def __init__(self) -> None:
        "Creates a 3x3 Rubik's Cube"
    def reset(self) -> None:
        "Resets the cube to a solved state"
    def is_solved(self) -> bool:
        "Returns `True` if the cube is solved, and `False` otherwise"
    def get_observations(self) -> list[int]: ...
    def apply_action(self, action: int) -> None: ...
    def scramble(self, action: int) -> int: ...
    def scramble_with_seed(self, seed: int, steps: int) -> None: ...
