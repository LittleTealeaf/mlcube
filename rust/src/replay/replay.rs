use rand::Rng;

use crate::puzzle::{ApplyActionError, Puzzle};

#[derive(Clone)]
pub struct ReplayEntry {
    pub current_state: Vec<u8>,
    pub action: usize,
    pub reward: f64,
    pub next_state: Vec<u8>,
}

pub struct Replay<T: Puzzle> {
    puzzle: T,
    data: Vec<ReplayEntry>,
    capacity: usize,
}

impl<T: Puzzle> Replay<T> {
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            puzzle: T::default(),
            data: Vec::with_capacity(capacity),
            capacity,
        }
    }

    pub fn record_action(&mut self, action: usize, reward: f64) -> Result<(), RecordActionError> {
        let current_state = self.get_observations();
        self.apply_action(action)?;
        let next_state = self.get_observations();

        if self.data.len() == self.capacity {
            let mut rng = rand::thread_rng();
            let index = rng.gen_range(0..(self.capacity));
            self.data.swap_remove(index);
        }

        self.data.push(ReplayEntry {
            current_state,
            action,
            reward,
            next_state,
        });

        Ok(())
    }

    pub fn sample_replay(&mut self, count: usize) -> Result<Vec<ReplayEntry>, SampleReplayError> {
        if self.data.len() == 0 {
            return Err(SampleReplayError::EmptyReplay);
        }

        let mut replay = Vec::new();

        let mut rng = rand::thread_rng();

        for _ in 0..count {
            let index = rng.gen_range(0..self.data.len());
            let instance = self.data[index].clone();
            replay.push(instance);
        }

        Ok(replay)
    }

    pub fn is_at_capacity(&self) -> bool {
        self.data.len() == self.capacity
    }
}

impl<T: Puzzle> Puzzle for Replay<T> {
    const OBSERVATION_SIZE: usize = T::OBSERVATION_SIZE;

    const ACTION_SIZE: usize = T::ACTION_SIZE;

    /// Deprecated: Please use apply_replay_action(action, reward) instead
    fn apply_action(&mut self, action: usize) -> Result<(), ApplyActionError> {
        self.puzzle.apply_action(action)
    }

    fn get_observations(&self) -> Vec<u8> {
        self.puzzle.get_observations()
    }

    fn reset(&mut self) {
        self.puzzle.reset()
    }

    fn is_solved(&self) -> bool {
        self.puzzle.is_solved()
    }
}

impl<T: Puzzle> Default for Replay<T> {
    fn default() -> Self {
        Self {
            puzzle: T::default(),
            data: Vec::with_capacity(100_000),
            capacity: 100_000,
        }
    }
}

#[derive(Debug)]
pub enum RecordActionError {
    ApplyActionError(ApplyActionError),
}

impl From<ApplyActionError> for RecordActionError {
    fn from(value: ApplyActionError) -> Self {
        Self::ApplyActionError(value)
    }
}

pub enum SampleReplayError {
    EmptyReplay,
}

#[cfg(test)]
mod tests {
    use crate::puzzle::puzzles::{Cube2x2, Cube3x3};

    use super::*;

    #[test]
    fn is_at_capacity() {
        let mut replay = Replay::<Cube2x2>::default();
        let capacity = replay.capacity;

        let mut rng = rand::thread_rng();

        for _ in 0..capacity {
            assert!(!replay.is_at_capacity());
            let action = rng.gen_range(0..Replay::<Cube2x2>::ACTION_SIZE);
            replay.record_action(action, rng.gen()).unwrap();
        }
        assert!(replay.is_at_capacity());
    }

    #[test]
    fn vector_length_stays_at_capacity() {
        let mut replay = Replay::<Cube3x3>::default();
        let capacity = replay.capacity;

        let mut rng = rand::thread_rng();
        for _ in 0..(capacity + 1) {
            let action = rng.gen_range(0..Replay::<Cube3x3>::ACTION_SIZE);
            replay.record_action(action, 0.0).unwrap();
        }
    }

    #[test]
    fn latest_record_is_always_inserted() {
        let mut replay = Replay::<Cube3x3>::default();
        let capacity = replay.capacity;

        let mut rng = rand::thread_rng();

        for _ in 0..(capacity * 2) {
            let action = rng.gen_range(0..Replay::<Cube3x3>::ACTION_SIZE);
            let obs = replay.get_observations();
            replay.record_action(action, 0.0).unwrap();

            let mut found = false;
            for item in replay.data.iter().rev() {
                if item.current_state.eq(&obs) {
                    found = true;
                    break;
                }
            }
            assert!(found, "Latest entry was not found");
        }
    }

    #[test]
    fn data_capacity_is_not_increased() {
        let mut replay = Replay::<Cube2x2>::default();
        let capacity = replay.capacity;

        let mut rng = rand::thread_rng();

        for _ in 0..(capacity + 5) {
            let action = rng.gen_range(0..(Replay::<Cube2x2>::ACTION_SIZE));
            replay.record_action(action, 0.0).unwrap();
            assert_eq!(capacity, replay.data.capacity());
        }
    }

    #[test]
    fn sampling_empty_data_returns_error() {
        let mut replay = Replay::<Cube3x3>::default();

        assert!(match replay.sample_replay(10) {
            Ok(_) => false,
            Err(error) => match error {
                SampleReplayError::EmptyReplay => true,
            },
        });
    }

    #[test]
    fn record_action_scrambles_cube_correctly() {
        for action in 0..(Replay::<Cube3x3>::ACTION_SIZE) {
            let mut replay = Replay::<Cube3x3>::default();
            let mut cube = Cube3x3::default();

            replay.record_action(action, 0.0).unwrap();
            cube.apply_action(action).unwrap();

            let obs_replay = replay.get_observations();
            let obs_cube = cube.get_observations();

            assert_eq!(obs_replay, obs_cube);
        }
    }

    #[test]
    fn record_invalid_action_returns_error() {
        let mut replay = Replay::<Cube3x3>::default();
        let result = replay.record_action(Replay::<Cube3x3>::ACTION_SIZE, 0.0);
        assert!(match result {
            Ok(_) => false,
            Err(err) => match err {
                RecordActionError::ApplyActionError(_) => true,
            },
        })
    }

    mod puzzle_trait {
        use crate::puzzle::puzzles::Cube2x2;

        use super::*;

        #[test]
        fn observations_has_correct_size() {
            let replay = Replay::<Cube2x2>::default();
            let observations = replay.get_observations();
            let observation_length = observations.len();
            assert_eq!(
                observation_length,
                Cube2x2::OBSERVATION_SIZE,
                "Observations should be of length {}, found {}",
                Cube2x2::OBSERVATION_SIZE,
                observation_length
            );
        }

        #[test]
        fn observations_have_valid_values() {
            let replay = Replay::<Cube2x2>::default();
            for value in replay.get_observations() {
                assert!(
                    value == 0 || value == 1,
                    "Any value in cube should be 0 or 1, found {}",
                    value
                );
            }
        }

        #[test]
        fn default_cube_is_solved() {
            let replay = Replay::<Cube2x2>::default();
            assert!(
                replay.is_solved(),
                "A default cube should be solved, found unsolved cube"
            );
        }

        #[test]
        fn applying_move_makes_cube_unsolved() {
            for i in 0..18 {
                let mut replay = Replay::<Cube2x2>::default();
                replay.apply_action(i).unwrap();

                assert!(
                    !replay.is_solved(),
                    "Applying the action {} should be unsolved, found a solved cube",
                    i
                );
            }
        }

        #[test]
        fn repeat_moves_loops_to_solved() {
            for i in 0..18 {
                let mut replay = Replay::<Cube2x2>::default();
                replay.apply_action(i).unwrap();
                replay.apply_action(i).unwrap();
                replay.apply_action(i).unwrap();
                replay.apply_action(i).unwrap();
                assert!(
                    replay.is_solved(),
                    "Applying the action {} four times should be solved, found an unsolved cube",
                    i
                );
            }
        }

        #[test]
        fn reset_solved_cube_is_solved() {
            let mut replay = Replay::<Cube2x2>::default();
            replay.reset();
            assert!(
                replay.is_solved(),
                "A cube should be solved after resetting it"
            );
        }

        #[test]
        fn reset_unsolved_cube_is_solved() {
            for i in 0..18 {
                let mut replay = Replay::<Cube2x2>::default();
                replay.apply_action(i).unwrap();
                replay.reset();
                assert!(
                    replay.is_solved(),
                    "A cube unsolved by the action {} should be solved after a reset",
                    i
                );
            }
        }

        #[test]
        fn invalid_action_returns_error() {
            let mut replay = Replay::<Cube2x2>::default();
            assert!(
                replay.apply_action(Cube2x2::ACTION_SIZE).is_err(),
                "Applying the action {} should return an Err because {} is an invalid action",
                Cube2x2::ACTION_SIZE,
                Cube2x2::ACTION_SIZE
            );
        }

        #[test]
        fn observations_have_valid_format() {
            let replay = Replay::<Cube2x2>::default();
            let observations = replay.get_observations();
            for segment in 0..(4 * 6) {
                let start_index = segment * 6;
                let slice = &observations[start_index..(start_index + 6)];

                let sum: u8 = slice.iter().sum();

                assert!(
                    sum > 0,
                    "Invalid slice from {} to {}, did not find any positive value",
                    start_index,
                    start_index + 6
                );

                assert!(
                    sum < 2,
                    "Invalid slice from {} to {}, too many positive values found",
                    start_index,
                    start_index + 6
                );
            }
        }

        #[test]
        fn scramble_with_seed_unsolves_cube() {
            let mut replay = Replay::<Cube2x2>::default();
            let seed = 1234;
            replay.scramble_with_seed(100, seed);
            assert!(!replay.is_solved());
        }

        #[test]
        fn scramble_unsolves_cube() {
            let mut replay = Replay::<Cube2x2>::default();
            replay.scramble(100);
            assert!(!replay.is_solved());
        }

        #[test]
        fn scramble_seeds_are_random() {
            let mut visited_seeds = Vec::new();
            for _ in 0..100 {
                let mut replay = Replay::<Cube2x2>::default();
                let seed = replay.scramble(10);
                assert!(
                    !visited_seeds.contains(&seed),
                    "Duplicate Seed Found: {}",
                    seed
                );
                visited_seeds.push(seed);
            }
        }
    }
}
