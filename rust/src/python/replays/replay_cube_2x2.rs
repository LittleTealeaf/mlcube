use pyo3::{pyclass, pymethods, PyResult};

use crate::{
    puzzle::{puzzles::Cube2x2, Puzzle},
    replay::Replay,
};

use super::PyReplayEntry;

type Replay2x2 = Replay<Cube2x2>;

#[pyclass]
pub struct PyReplay2x2 {
    replay: Replay2x2,
}

#[pymethods]
impl PyReplay2x2 {
    #[new]
    #[pyo3(signature = (capacity = 100_000))]
    fn new(capacity: usize) -> Self {
        Self {
            replay: Replay::with_capacity(capacity),
        }
    }

    #[classattr]
    fn observation_length() -> usize {
        Replay2x2::OBSERVATION_SIZE
    }

    #[classattr]
    fn action_size() -> usize {
        Replay2x2::ACTION_SIZE
    }

    fn record_action(&mut self, action: usize) -> PyResult<()> {
        self.replay.record_action(action)?;
        Ok(())
    }

    fn sample_replay(&mut self, count: usize) -> PyResult<Vec<PyReplayEntry>> {
        Ok(self
            .replay
            .sample_replay(count)?
            .into_iter()
            .map(PyReplayEntry::from)
            .collect())
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

    fn is_at_capacity(&self) -> bool {
        self.replay.is_at_capacity()
    }

    fn get_observations(&self) -> Vec<u8> {
        self.replay.get_observations()
    }
}

impl Default for PyReplay2x2 {
    fn default() -> Self {
        Self {
            replay: Replay::default(),
        }
    }
}
