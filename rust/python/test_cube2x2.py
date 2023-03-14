from librust import PyCube2x2

def test_observation_size():
    assert PyCube2x2.observation_length > 0

    cube = PyCube2x2()
    observations = cube.get_observations()

    assert len(observations) == PyCube2x2.observation_length

    for value in observations:
        assert value == 0 or value == 1

def test_new_cube_is_solved():
    cube = PyCube2x2()
    assert cube.is_solved()

def test_applying_move_unsolves_cube():
    for i in range(PyCube2x2.action_size):
        cube = PyCube2x2()
        cube.apply_action(i)
        assert not cube.is_solved()

def test_repeat_moves_to_solved():
    for i in range(18):
        cube = PyCube2x2()
        cube.apply_action(i)
        cube.apply_action(i)
        cube.apply_action(i)
        cube.apply_action(i)
        assert cube.is_solved()

def test_reset_solves_cube():
    for i in range(18):
        cube = PyCube2x2()
        cube.apply_action(i)
        cube.reset()
        assert cube.is_solved()

def test_get_observations_are_unique():
    for i in range(18):
        cube = PyCube2x2()
        obs_a = cube.get_observations()
        cube.apply_action(i)
        obs_b = cube.get_observations()

        same = True
        for i in range(PyCube2x2.observation_length):
            if obs_a[i] != obs_b[i]:
                same = False
        assert not same

def test_scramble_unsolves_cube():
    cube = PyCube2x2()
    cube.scramble(100)
    assert not cube.is_solved()

def test_scramble_seeds_repeatable():
    seed = 34546
    a = PyCube2x2()
    a.scramble_with_seed(seed, 100)
    b = PyCube2x2()
    b.scramble_with_seed(seed, 100)

    obs_a = a.get_observations()
    obs_b = b.get_observations()

    for i in range(PyCube2x2.observation_length):
        assert obs_a[i] == obs_b[i]

def test_scramble_returns_correct_seed():
    cube_a = PyCube2x2()
    seed = cube_a.scramble(100)

    cube_b = PyCube2x2()
    cube_b.scramble_with_seed(seed, 100)

    obs_a = cube_a.get_observations()
    obs_b = cube_b.get_observations()

    for i in range(PyCube2x2.observation_length):
        assert obs_a[i] == obs_b[i]

