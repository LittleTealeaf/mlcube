from mlcube import PyCube3x3

def test_observation_size():
    assert PyCube3x3.observation_length > 0

    cube = PyCube3x3()
    observations = cube.get_observations()

    assert len(observations) == PyCube3x3.observation_length

    for value in observations:
        assert value == 0 or value == 1

def test_new_cube_is_solved():
    cube = PyCube3x3()
    assert cube.is_solved()

def test_applying_move_unsolves_cube():
    for i in range(PyCube3x3.action_size):
        cube = PyCube3x3()
        cube.apply_action(i)
        assert not cube.is_solved()

def test_repeat_moves_to_solved():
    for i in range(18):
        cube = PyCube3x3()
        cube.apply_action(i)
        cube.apply_action(i)
        cube.apply_action(i)
        cube.apply_action(i)
        assert cube.is_solved()

def test_reset_solves_cube():
    for i in range(18):
        cube = PyCube3x3()
        cube.apply_action(i)
        cube.reset()
        assert cube.is_solved()

def test_get_observations_are_unique():
    for i in range(18):
        cube = PyCube3x3()
        obs_a = cube.get_observations()
        cube.apply_action(i)
        obs_b = cube.get_observations()

        same = True
        for i in range(PyCube3x3.observation_length):
            if obs_a[i] != obs_b[i]:
                same = False
        assert not same

def test_scramble_unsolves_cube():
    cube = PyCube3x3()
    cube.scramble(100)
    assert not cube.is_solved()

def test_scramble_seeds_repeatable():
    seed = 34546
    a = PyCube3x3()
    a.scramble_with_seed(seed)
    b = PyCube3x3()
    b.scramble_with_seed(seed)

    obs_a = a.get_observations()
    obs_b = b.get_observations()

    for i in range(PyCube3x3.observation_length):
        assert obs_a[i] == obs_b[i]

def test_scramble_returns_correct_seed():
    cube_a = PyCube3x3()
    seed = cube_a.scramble()

    cube_b = PyCube3x3()
    cube_b.scramble_with_seed(seed)

    obs_a = cube_a.get_observations()
    obs_b = cube_b.get_observations()

    for i in range(PyCube3x3.observation_length):
        assert obs_a[i] == obs_b[i]
