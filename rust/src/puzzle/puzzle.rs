use rand::prelude::Rng;
use rand::SeedableRng;
use rand_chacha::ChaCha8Rng;

#[derive(Debug)]
pub enum ApplyActionError {
    InvalidActionIndex,
}

/// Implements functions that every puzzle should be able to manage
pub trait Puzzle: Default {
    const OBSERVATION_SIZE: usize;
    const ACTION_SIZE: usize;

    fn apply_action(&mut self, action: usize) -> Result<(), ApplyActionError>;

    fn get_observations(&self) -> Vec<u8>;

    fn reset(&mut self);

    fn is_solved(&self) -> bool;

    fn scramble_with_seed(&mut self, steps: usize, seed: u64) {
        let mut rnd = ChaCha8Rng::seed_from_u64(seed);

        for _ in 0..steps {
            let action = rnd.gen_range(0..Self::ACTION_SIZE);
            self.apply_action(action).unwrap();
        }
    }

    fn scramble(&mut self, steps: usize) -> u64 {
        let mut rng = ChaCha8Rng::from_entropy();
        let seed = rng.gen();
        self.scramble_with_seed(steps, seed);
        seed
    }
}
