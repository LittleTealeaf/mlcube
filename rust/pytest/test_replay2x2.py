from mlcube import PyReplay2x2

def test_observation_size():
    assert PyReplay2x2.observation_length > 0

    cube = PyReplay2x2(100)
    observations = cube.get_observations()

    assert len(observations) == PyReplay2x2.observation_length

    for value in observations:
        assert value == 0 or value == 1

def test_new_replay_is_solved():
    replay = PyReplay2x2(100)
    assert replay.is_solved()

def test_applying_move_unsolves_replay():
    for i in range(PyReplay2x2.action_size):
        replay = PyReplay2x2(100)
        replay.apply_action(i)
        assert not replay.is_solved()

def test_repeat_moves_to_solved():
    replay = PyReplay2x2(100)
    for i in range(replay.action_size):
        replay.apply_action(i)
        replay.apply_action(i)
        replay.apply_action(i)
        replay.apply_action(i)
        assert replay.is_solved()

def test_reset_solves_cube():
    replay = PyReplay2x2(100)
    for i in range(replay.action_size):
        replay.apply_action(i)
        replay.reset()
        assert replay.is_solved()
