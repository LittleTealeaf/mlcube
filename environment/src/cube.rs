use cpython::{exc, PyErr, PyResult};

py_class!(pub class Cube |py| {
  data state: [u8; 54];

  def __new__(_cls) -> PyResult<Cube> {
      let mut state = [0; 54];
      for i in 0..54 {
          state[i] = (i as u8) / 9;
      }
      Cube::create_instance(py, state)
  }

  def apply_move(&self, action: usize) -> PyResult<usize> {
    if action >= 6 * 3 {
      return Err(PyErr::new::<exc::TypeError, _>(py, "Move Index Out of Bounds"));
    }


    Ok(action)
  }

  def get_observations(&self) -> PyResult<Vec<u8>> {
      let mut observations = [0; 54 * 6];
      for i in 0..54 {
          observations[i * 6 + self.state(py)[i] as usize] = 1;
      }
      Ok(Vec::from(observations))
  }
});

fn get_permutations(face_index: usize) -> [[usize; 4]; 5] {
    match face_index {
        0 => [
            [20, 2, 42, 47],
            [23, 5, 39, 50],
            [26, 8, 36, 53],
            [27, 29, 35, 33],
            [28, 32, 34, 30],
        ],
        1 => [
            [20, 11, 38, 29],
            [19, 10, 37, 28],
            [18, 9, 36, 27],
            [8, 6, 0, 2],
            [7, 3, 1, 5],
        ],
        2 => [
            [6, 27, 47, 17],
            [7, 30, 46, 14],
            [8, 33, 45, 11],
            [18, 20, 26, 24],
            [19, 23, 25, 21],
        ],
        3 => [
            [18, 45, 44, 0],
            [21, 48, 41, 3],
            [24, 51, 38, 6],
            [11, 17, 15, 9],
            [14, 16, 12, 10],
        ],
        4 => [
            [24, 33, 42, 15],
            [25, 34, 43, 16],
            [26, 35, 44, 17],
            [45, 47, 53, 51],
            [46, 50, 52, 48],
        ],
        _ => [
            [36, 38, 44, 42],
            [37, 41, 43, 39],
            [29, 0, 15, 53],
            [32, 1, 12, 52],
            [35, 2, 9, 51],
        ],
    }
}
