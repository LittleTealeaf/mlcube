use cpython::PyResult;

mod sim;

#[macro_use]
extern crate cpython;

py_module_initializer!(librust, |py, m| {
    m.add(py, "__doc__", "This module is implemented in rust")?;
    Ok(())
});
