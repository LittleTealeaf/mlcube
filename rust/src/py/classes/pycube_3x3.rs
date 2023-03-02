use std::cell::RefCell;

use cpython::{exc, PyErr, PyNone, PyResult};

use crate::puzzle::{puzzles::Cube3x3, Puzzle};

py_class!(pub class PyCube3x3 |py| {
  data cube: RefCell<Cube3x3>;

  def __new__(_cls) -> PyResult<PyCube3x3> {
    PyCube3x3::create_instance(py, RefCell::new(Cube3x3::default()))
  }

  def get_observation_length(&self) -> PyResult<usize> {
    Ok(Cube3x3::OBSERVATION_SIZE)
  }

  def reset(&self) -> PyResult<PyNone> {
    self.cube(py).borrow_mut().reset();
    Ok(PyNone)
  }

  def is_solved(&self) -> PyResult<bool> {
    Ok(self.cube(py).borrow().is_solved())
  }

  def apply_action(&self, action: usize) -> PyResult<PyNone> {
    match self.cube(py).borrow_mut().apply_action(action) {
      Ok(_) => Ok(PyNone),
      Err(_) => Err(PyErr::new::<exc::IndexError, _>(py, "Action Index is out of bounds"))
    }
  }

  def get_observations(&self) -> PyResult<Vec<u8>> {
    Ok(self.cube(py).borrow().get_observations())
  }
});
