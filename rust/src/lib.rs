use std::cell::RefCell;

use cpython::PyResult;
use sim::Cube;

mod sim;

#[macro_use]
extern crate cpython;

py_module_initializer!(librust, |py, m| {
    m.add(py, "__doc__", "This module is implemented in rust")?;
    Ok(())
});

py_class!(class PyCube |py| {
    data cube: RefCell<Cube>;

    def __new__(_cls) -> PyResult<PyCube> {
        PyCube::create_instance(py, RefCell::new(Cube::default()))
    }
});
