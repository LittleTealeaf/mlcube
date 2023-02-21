use cpython::{exc, PyErr, PyNone, PyResult};
use cube::Cube;
use std::cell::RefCell;

mod cube;

#[macro_use]
extern crate cpython;

py_module_initializer!(libenvironment, |py, m| {
    m.add(py, "__doc__", "This module is implemented in rust")?;
    m.add_class::<PyCube>(py)?;
    Ok(())
});

py_class!(class PyCube |py| {
    data cube: RefCell<Cube>;

    def __new__(_cls) -> PyResult<PyCube> {
        PyCube::create_instance(py, RefCell::new(Cube::new()))
    }

    def apply_action(&self, action: usize) -> PyResult<PyNone> {
        match self.cube(py).borrow_mut().apply_action(action) {
            Some(_) => Ok(PyNone),
            None => Err(PyErr::new::<exc::IndexError, _>(py, "Action Index Out of Bounds. Actions must be in the range [0,18]"))
        }
    }

    def get_observations(&self) -> PyResult<Vec<u8>> {
        Ok(Vec::from(self.cube(py).borrow().get_observations()))
    }

    def reset(&self) -> PyResult<PyNone> {
        self.cube(py).borrow_mut().reset();
        Ok(PyNone)
    }

    def is_solved(&self) -> PyResult<bool> {
        Ok(self.cube(py).borrow().is_solved())
    }
});
