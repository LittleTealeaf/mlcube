use rand::prelude::Rng;
use rand::SeedableRng;
use rand_chacha::ChaCha8Rng;

/// Exposes variables and functions generalized for any puzzle
pub trait Puzzle: Default {
    /// The length of the array that will be returned from the `get_observations()` method.
    const OBSERVATION_SIZE: usize;
    /// The number of possible actions that can be performed on the puzzle.
    const ACTION_SIZE: usize;

    fn new() -> Self {
        Self::default()
    }

    /// Attempts to apply an action to the puzzle.
    ///
    /// Parameters
    /// ----------
    /// action: `usize`
    ///     The action to apply to the puzzle. Each puzzle will have a different number of possible
    ///     actions, which can be found from `Puzzle::ACTION_SIZE`
    ///
    /// Returns
    /// -------
    /// `Result<(), ApplyActionIndex>`
    ///     Returns `()` if the action was applied successfully. If there was an error, it will
    ///     return an `ApplyActionError`.
    fn apply_action(&mut self, action: usize) -> Result<(), ApplyActionError>;

    /// Compiles the puzzle into an observation array where every value is either `0` or `1`.
    ///
    /// Returns
    /// -------
    /// `Vec<u8>`
    ///     Returns a vector of length `Puzzle::OBSERVATION_SIZE` of `0` and `1`s indicating the
    ///     puzzle's unique state.
    fn get_observations(&self) -> Vec<u8>;

    /// Resets the puzzle to it's default state
    fn reset(&mut self);

    /// Checks if the puzzle is solved
    ///
    /// Returns
    /// -------
    /// `bool`
    ///     `true` if the puzzle is solved, `false` otherwise
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

    fn get_reward(&self) -> f64;
}

#[derive(Debug)]
pub enum ApplyActionError {
    InvalidActionIndex,
}
