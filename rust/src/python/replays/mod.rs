mod replay_cube_2x2;
pub use replay_cube_2x2::*;
mod replay_cube_3x3;
pub use replay_cube_3x3::*;


pub(crate) type PyReplaySample = (Vec<Vec<u8>>, Vec<usize>, Vec<f64>, Vec<Vec<u8>>);

use pyo3::{exceptions::PyValueError, PyErr};

use crate::replay::{ReplayEntry, SampleReplayError};

impl From<SampleReplayError> for PyErr {
    fn from(value: SampleReplayError) -> Self {
        match value {
            SampleReplayError::EmptyReplay => PyValueError::new_err("Empty Replay"),
        }
    }
}

pub fn sample_from_set(sample: Vec<ReplayEntry>) -> PyReplaySample {
    let capacity = sample.len();
    let mut current_state = Vec::with_capacity(capacity);
    let mut action = Vec::with_capacity(capacity);
    let mut reward = Vec::with_capacity(capacity);
    let mut next_state = Vec::with_capacity(capacity);

    for entry in sample {
        current_state.push(entry.current_state);
        action.push(entry.action);
        reward.push(entry.reward);
        next_state.push(entry.next_state);
    }


    (current_state, action, reward, next_state)
}
