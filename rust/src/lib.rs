use std::cell::RefCell;

use cpython::{exc, PyErr, PyNone, PyResult};
use sim::{Action, Cube};

mod sim;

#[macro_use]
extern crate cpython;

py_module_initializer!(librust, |py, m| {
    m.add(py, "__doc__", "This module is implemented in rust")?;
    m.add_class::<PyCube>(py)?;
    Ok(())
});

py_class!(class PyCube |py| {
    data cube: RefCell<Cube>;

    def __new__(_cls) -> PyResult<PyCube> {
        PyCube::create_instance(py, RefCell::new(Cube::default()))
    }

    def apply_action(&self, action_index: usize) -> PyResult<PyNone> {

        match Action::try_from(action_index) {
            Ok(action) => {
                self.cube(py).borrow_mut().apply_action(&action);
                Ok(PyNone)
            },
            Err(_) => Err(PyErr::new::<exc::IndexError, _>(py, "Action Index Out of Bounds"))
        }
    }
});
