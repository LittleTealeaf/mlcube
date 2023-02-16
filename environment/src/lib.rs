mod action;
mod cube;



use crate::cube::Cube;

#[macro_use]
extern crate cpython;

py_module_initializer!(libenvironment, |py, m| {
    m.add(py, "__doc__", "This module is implemented in rust")?;
    m.add_class::<Cube>(py)?;
    Ok(())
});
