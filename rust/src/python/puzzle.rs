use pyo3::{PyErr, exceptions::PyValueError};

use crate::puzzle::ApplyActionError;


impl From<ApplyActionError> for PyErr {
    fn from(_: ApplyActionError) -> Self {
        PyValueError::new_err("Invalid Apply Action Value")
    }
}
