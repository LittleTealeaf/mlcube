use pyo3::{types::PyModule, PyResult, Python, pymodule};
use python::{PyCube2x2, PyCube3x3, PyReplay2x2};

pub mod puzzle;
pub mod python;
pub mod replay;


/// This module is implemented in rust
#[pymodule]
fn mlcube(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
    m.add_class::<PyCube2x2>()?;
    m.add_class::<PyCube3x3>()?;
    m.add_class::<PyReplay2x2>()?;
    Ok(())
}
