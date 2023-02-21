import libenvironment

def test_create_cube_is_not_null():
  cube = libenvironment.PyCube()
  assert cube != None

def test_new_cube_is_solved():
  cube = libenvironment.PyCube()
  assert cube.is_solved()
