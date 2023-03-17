mod py_cube_2x2;
pub use py_cube_2x2::*;

mod py_cube_3x3;
pub use py_cube_3x3::*;
use pyo3::{PyErr, exceptions::PyValueError};

use crate::puzzle::ApplyActionError;


impl From<ApplyActionError> for PyErr {
    fn from(_: ApplyActionError) -> Self {
        PyValueError::new_err("Invalid Apply Action Value")
    }
}
