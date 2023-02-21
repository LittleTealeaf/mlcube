use cpython::{PyResult, PyErr, exc, Python};
use traits::Indexable;
use sim::Action;

mod sim;
mod traits;

#[macro_use]
extern crate cpython;


py_module_initializer!(librust, |py, m| {
  m.add(py, "__doc__", "This module is implemented in rust")?;
  m.add(py, "get_action_index", py_fn!(py, py_get_action_index(name: &str)))?;
  Ok(())
});


fn py_get_action_index(py: Python, name: &str) -> PyResult<usize> {
  match name.parse::<Action>() {
    Ok(action) => Ok(action.to_index()),
    Err(_) => Err(PyErr::new::<exc::IOError, _>(py, "Could not parse action"))
  }
}
