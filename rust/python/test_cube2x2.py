# type: ignore
import librust

def test_create_cube():
  cube = librust.PyCube2x2()
  assert cube is not None

def test_new_cube_is_solved():
  cube = librust.PyCube2x2()
  assert cube.is_solved()

def test_apply_valid_action():
  for i in range(18):
    cube = librust.PyCube2x2()
    cube.apply_action(i)
    assert not cube.is_solved()

def test_get_observation_length():
  cube = librust.PyCube2x2()
  assert type(cube.get_observation_length()) == int
  assert cube.get_observation_length() > 0

def test_get_observations_is_of_length():
  cube = librust.PyCube2x2()
  length = cube.get_observation_length()
  assert len(cube.get_observations()) == length

def test_reset():
    for i in range(18):
        cube = librust.PyCube2x2()
        cube.apply_action(i)
        cube.reset()
        assert cube.is_solved()

def test_scramble_unsovles_cube():
    cube = librust.PyCube2x2()
    cube.scramble(100)
    assert not cube.is_solved()

def test_scramble_with_seed_unsolves_cube():
   cube = librust.PyCube2x2()
   cube.scramble_with_seed(100,12345)
   assert not cube.is_solved()

def test_scramble_seeds_produce_identical_cubes():

   seed = 12345

   cube_a = librust.PyCube2x2()
   cube_a.scramble_with_seed(100,seed)

   cube_b = librust.PyCube2x2()
   cube_b.scramble_with_seed(100,seed)

   obs_a = cube_a.get_observations()
   obs_b = cube_b.get_observations()

   for i in range(cube_a.get_observation_length()):
      assert obs_a[i] == obs_b[i]

def test_scramble_returns_correct_seed():
   cube_a = librust.PyCube2x2()
   seed = cube_a.scramble(100)

   cube_b = librust.PyCube2x2()
   cube_b.scramble_with_seed(100,seed)

   obs_a = cube_a.get_observations()
   obs_b = cube_b.get_observations()

   for i in range(cube_a.get_observation_length()):
      assert obs_a[i] == obs_b[i]
