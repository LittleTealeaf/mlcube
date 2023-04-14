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

#[cfg(test)]
mod tests {
    use crate::{
        puzzle::{puzzles::Cube3x3, Puzzle},
        replay::Replay,
    };

    use super::*;

    #[test]
    fn sample_from_set_keeps_minimum_capacity() {
        let mut replay = Replay::<Cube3x3>::with_capacity(1000);
        for i in 0..1000 {
            replay.apply_action(i % 18).unwrap();
        }
        let sample = {
            match replay.sample_replay(500) {
                Ok(t) => t,
                Err(_) => panic!(),
            }
        };

        let (current, action, reward, next) = sample_from_set(sample);

        assert_eq!(current.capacity(), 500);
        assert_eq!(action.capacity(), 500);
        assert_eq!(reward.capacity(), 500);
        assert_eq!(next.capacity(), 500);
    }
}
