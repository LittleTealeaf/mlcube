use std::cell::RefCell;

use cpython::{exc, PyErr, PyNone, PyResult};

use crate::puzzle::{puzzles::Cube2x2, Puzzle};

py_class!(pub class PyCube2x2 |py| {

    data cube: RefCell<Cube2x2>;

    def __new__(_cls) -> PyResult<PyCube2x2> {
        PyCube2x2::create_instance(py, RefCell::new(Cube2x2::default()))
    }

    def get_observation_length(&self) -> PyResult<usize> {
        Ok(Cube2x2::OBSERVATION_SIZE)
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
