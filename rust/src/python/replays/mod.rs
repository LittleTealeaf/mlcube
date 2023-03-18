mod replay_cube_2x2;
pub use replay_cube_2x2::*;
mod replay_cube_3x3;
pub use replay_cube_3x3::*;

pub(crate) type PyReplayEntry = (Vec<u8>, usize, f64, Vec<u8>);

use pyo3::{exceptions::PyValueError, PyErr};

use crate::replay::{RecordActionError, ReplayEntry, SampleReplayError};

impl From<RecordActionError> for PyErr {
    fn from(error: RecordActionError) -> Self {
        match error {
            RecordActionError::ApplyActionError(error) => error.into(),
        }
    }
}

impl From<ReplayEntry> for PyReplayEntry {
    fn from(value: ReplayEntry) -> Self {
        (
            value.current_state,
            value.action,
            value.reward,
            value.next_state,
        )
    }
}

impl From<SampleReplayError> for PyErr {
    fn from(value: SampleReplayError) -> Self {
        match value {
            SampleReplayError::EmptyReplay => PyValueError::new_err("Empty Replay"),
        }
    }
}
