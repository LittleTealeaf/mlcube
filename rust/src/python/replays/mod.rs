mod replay_cube_2x2;
pub use replay_cube_2x2::*;
mod replay_cube_3x3;
pub use replay_cube_3x3::*;



pub(crate) type PyReplayEntry =  (Vec<u8>, usize, f64, Vec<u8>);

use pyo3::{PyErr, exceptions::PyValueError};

use crate::replay::{RecordActionError, ReplayEntry};

impl From<RecordActionError> for PyErr {
    fn from(_: RecordActionError) -> Self {
        PyValueError::new_err("Invalid Apply Action Value")
    }
}

impl From<ReplayEntry> for PyReplayEntry {
    fn from(value: ReplayEntry) -> Self {
        (
            value.current_state,
            value.action,
            value.reward,
            value.next_state
        )
    }
}
