use rand::{rngs::ThreadRng, seq::IteratorRandom, Rng};
use serde::{Deserialize, Serialize};

use crate::puzzle::Puzzle;

#[derive(Clone, Copy, Serialize, Deserialize)]
pub struct ReplayObservation<P>
where
    P: Puzzle,
{
    pub state: P,
    pub action: usize,
    pub reward: f64,
    pub next_state: P,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct ReplayBuffer<P>
where
    P: Puzzle,
{
    buffer: Vec<ReplayObservation<P>>,
    max_size: usize,
}

impl<P> ReplayBuffer<P>
where
    P: Puzzle,
{
    pub fn new(max_size: usize) -> Self {
        Self {
            buffer: Vec::with_capacity(max_size),
            max_size,
        }
    }

    pub fn insert_observation(&mut self, observation: ReplayObservation<P>, rng: &mut ThreadRng) {
        if self.buffer.len() == self.max_size {
            self.buffer[rng.gen_range(0..self.max_size)] = observation;
        } else {
            self.buffer.push(observation);
        }
    }

    pub fn sample(&self, count: usize, rng: &mut ThreadRng) -> Vec<ReplayObservation<P>> {
        self.buffer
            .iter()
            .choose_multiple(rng, count)
            .into_iter()
            .cloned()
            .collect()
    }
}
