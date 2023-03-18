use pyo3::{pyclass, pymethods, PyResult};

use crate::{
    puzzle::{puzzles::Cube3x3, Puzzle},
    replay::Replay,
};

use super::PyReplayEntry;

type Replay3x3 = Replay<Cube3x3>;

#[pyclass]
pub struct PyReplay3x3 {
    replay: Replay3x3,
}

#[pymethods]
impl PyReplay3x3 {
    #[new]
    fn new(capacity: usize) -> Self {
        Self {
            replay: Replay::with_capacity(capacity),
        }
    }

    #[classattr]
    fn observation_length() -> usize {
        Replay3x3::OBSERVATION_SIZE
    }

    #[classattr]
    fn action_size() -> usize {
        Replay3x3::ACTION_SIZE
    }

    fn record_action(&mut self, action: usize, reward: f64) -> PyResult<()> {
        self.replay.record_action(action, reward)?;
        Ok(())
    }

    fn sample_replay(&mut self, count: usize) -> Vec<PyReplayEntry> {
        self.replay
            .sample_replay(count)
            .into_iter()
            .map(PyReplayEntry::from)
            .collect()
    }

    fn scramble(&mut self, steps: usize) -> u64 {
        self.replay.scramble(steps)
    }

    fn scramble_with_seed(&mut self, steps: usize, seed: u64) {
        self.replay.scramble_with_seed(steps, seed)
    }

    fn reset(&mut self) {
        self.replay.reset()
    }

    fn is_solved(&mut self) -> bool {
        self.replay.is_solved()
    }

    fn apply_action(&mut self, action: usize) -> PyResult<()> {
        self.replay.apply_action(action)?;
        Ok(())
    }
}
