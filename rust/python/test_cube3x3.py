from librust import PyCube3x3 #type: ignore

def test_create_cube():
    cube = PyCube3x3()
    assert cube is not None

def test_new_cube_is_solved():
    cube = PyCube3x3()
    assert cube.is_solved()

def test_apply_valid_action():
    for i in range(18):
        cube = PyCube3x3()
        cube.apply_action(i)
        assert not cube.is_solved()

def test_get_observation_length():
    cube = PyCube3x3()
    length = cube.get_observation_length()
    observations = cube.get_observations()
    assert type(length) == int
    assert length > 0
    assert len(observations) == length

def test_reset():
    for i in range(18):
        cube = PyCube3x3()
        cube.apply_action(i)
        cube.reset()
        assert cube.is_solved()

def test_scramble_unsolves_cube():
    cube = PyCube3x3()
    cube.scramble(100)
    assert not cube.is_solved()

def test_scramble_with_seed_unsolves_cube():
    cube = PyCube3x3()
    cube.scramble_with_seed(110,12345)
    assert not cube.is_solved()

def test_scramble_seeds_produce_dientical_cubes():

    seed = 12345

    cube_a = PyCube3x3()
    cube_a.scramble_with_seed(100, seed)

    cube_b = PyCube3x3()
    cube_b.scramble_with_seed(100, seed)

    obs_a = cube_a.get_observations()
    obs_b = cube_b.get_observations()

    for i in range(cube_a.get_observation_length()):
        assert obs_a[i] == obs_b[i]
