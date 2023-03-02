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
