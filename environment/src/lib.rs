use cpython::{PyResult, Python};

#[macro_use]
extern crate cpython;

py_module_initializer!(libenvironment, |py, m| {
    m.add(py, "__doc__", "This module is implemented in rust")?;
    m.add(
        py,
        "sum_as_string",
        py_fn!(py, sum_as_string_py(a: i64, b: i64)),
    )?;
    
    Ok(())
});

fn sum_as_string(a: i64, b: i64) -> String {
    format!("{}", a + b).to_string()
}

fn sum_as_string_py(_: Python, a: i64, b: i64) -> PyResult<String> {
    let out = sum_as_string(a, b);
    Ok(out)
}
