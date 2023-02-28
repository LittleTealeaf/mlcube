use std::marker::PhantomData;

use crate::sim::{InvalidActionIndex, Puzzle, PuzzleTrait};

const DEFAULT_STATE: [usize; 24] = [
    0, 0, 0, 0, 1, 1, 1, 1, 2, 2, 2, 2, 3, 3, 3, 3, 4, 4, 4, 4, 5, 5, 5, 5,
];

const PERMUTATIONS: [[[usize; 4]; 3]; 6] = [
    // U
    [[0, 1, 3, 2], [8, 4, 16, 12], [9, 5, 17, 13]],
    // L
    [[0, 8, 20, 19], [2, 10, 22, 17], [5, 7, 6, 4]],
    // F
    [[8, 9, 11, 10], [5, 3, 14, 20], [7, 2, 12, 21]],
    // R
    [[14, 12, 13, 15], [9, 1, 18, 21], [11, 3, 16, 23]],
    // B
    [[16, 17, 19, 18], [13, 0, 6, 23], [15, 1, 4, 22]],
    // D
    [[20, 21, 23, 22], [10, 14, 18, 6], [11, 15, 19, 7]],
];

pub struct Cube2x2;

impl PuzzleTrait<Cube2x2> for Puzzle<Cube2x2> {
    const ACTION_SIZE: usize = 18;
    const OBSERVATION_LENGTH: usize = 4 * 6 * 6;
    const STATE_SIZE: usize = 6 * 4;

    fn apply_action(&mut self, action: usize) -> Result<(), InvalidActionIndex> {
        let permutations = PERMUTATIONS[action % 6];
        let rotation = action / 6;
        match rotation {
            0 => {
                // Normal
                for row in permutations {
                    let tmp = self.state[row[3]];
                    for i in 0..3 {
                        self.state[row[3 - i]] = self.state[row[2 - i]];
                    }
                    self.state[row[0]] = tmp;
                }

                Ok(())
            }
            1 => {
                // Prime
                for row in permutations {
                    let tmp = self.state[row[0]];
                    for i in 0..3 {
                        self.state[row[i]] = self.state[row[i + 1]];
                    }
                    self.state[row[3]] = tmp;
                }
                Ok(())
            }
            2 => {
                // Two
                for row in permutations {
                    for i in [0, 1] {
                        let tmp = self.state[row[i]];
                        self.state[row[i]] = self.state[row[i + 2]];
                        self.state[row[i + 2]] = tmp;
                    }
                }
                Ok(())
            }
            _ => Err(InvalidActionIndex),
        }
    }

    fn get_observations(&self) -> Vec<u8> {
        let mut observations = [0; Self::OBSERVATION_LENGTH];

        for i in 0..(24) {
            let value = self.state[i];
            observations[i * 6 + value] = 1;
        }

        Vec::from(observations)
    }

    fn is_solved(&self) -> bool {
        for i in 0..24 {
            if self.state[i] != i / 4 {
                return false;
            }
        }
        return true;
    }

    fn reset(&mut self) {
        self.state = Vec::from(DEFAULT_STATE);
    }
}

impl Default for Puzzle<Cube2x2> {
    fn default() -> Self {
        Self {
            state: Vec::from(DEFAULT_STATE),
            size: PhantomData::<Cube2x2>,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn permutations_contains_valid_indices() {
        for perms in PERMUTATIONS {
            for row in perms {
                for index in row {
                    assert!(index < 24);
                }
            }
        }
    }

    #[test]
    fn default_state_contains_valid_values() {
        for item in DEFAULT_STATE {
            assert!(item < 6);
        }
    }

    #[test]
    fn new_cube_is_solved() {
        let cube: Puzzle<Cube2x2> = Puzzle::default();
        assert!(cube.is_solved());
    }

    #[test]
    fn observations_have_correct_length() {
        let cube: Puzzle<Cube2x2> = Puzzle::default();
        let observations = cube.get_observations();
        assert_eq!(observations.len(), Puzzle::<Cube2x2>::OBSERVATION_LENGTH);
    }

    #[test]
    fn applying_action_makes_unsolved() {
        for i in 0..Puzzle::<Cube2x2>::ACTION_SIZE {
            let mut cube = Puzzle::<Cube2x2>::default();
            let action_applied = match cube.apply_action(i) {
                Ok(_) => true,
                Err(_) => false
            };
            assert!(action_applied);
            assert!(!cube.is_solved());
        }
    }
}
