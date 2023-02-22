use std::cell::RefCell;

use cpython::{exc, PyErr, PyNone, PyResult, Python};
use sim::{Action, Cube};
use traits::Indexable;

mod sim;
mod traits;

#[macro_use]
extern crate cpython;

py_module_initializer!(librust, |py, m| {
    m.add(py, "__doc__", "This module is implemented in rust")?;
    m.add(py, "parse_action", py_fn!(py, py_parse_action(name: &str)))?;
    Ok(())
});

fn py_parse_action(py: Python, name: &str) -> PyResult<usize> {
    match name.parse::<Action>() {
        Ok(action) => Ok(action.to_index()),
        Err(_) => Err(PyErr::new::<exc::IOError, _>(py, "Could not parse action")),
    }
}

py_class!(class PyCube |py| {
  data cube: RefCell<Cube>;

  def __new__(_cls) -> PyResult<PyCube> {
    PyCube::create_instance(py, RefCell::new(Cube::default()))
  }

  def apply_action(&self, action_index: usize) -> PyResult<PyNone> {
    let action = Action::from_index(action_index).ok_or(
      PyErr::new::<exc::IndexError, _>(
        py,
        "Action Index out of bounds. Actions must be in the range [0,18]"
      )
    )?;

    self.cube(py).borrow_mut().apply_action(action);

    Ok(PyNone)
  }
});
