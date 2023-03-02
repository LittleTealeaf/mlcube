use crate::py::{PyCube2x2, PyCube3x3};


mod py;
mod puzzle;

#[macro_use]
extern crate cpython;

py_module_initializer!(librust, |py, m| {
    m.add(py, "__doc__", "This module is implemented in rust")?;
    m.add_class::<PyCube2x2>(py)?;
    m.add_class::<PyCube3x3>(py)?;
    Ok(())
});
