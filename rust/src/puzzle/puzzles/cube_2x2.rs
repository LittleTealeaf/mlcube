use crate::puzzle::{ApplyActionError, Puzzle};

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

pub struct Cube2x2 {
    state: [usize; 24],
}

impl Default for Cube2x2 {
    fn default() -> Self {
        Self {
            state: DEFAULT_STATE,
        }
    }
}

impl Puzzle for Cube2x2 {
    const OBSERVATION_SIZE: usize = 4 * 6 * 6;
    const ACTION_SIZE: usize = 18;

    fn reset(&mut self) {
        self.state = DEFAULT_STATE;
    }

    fn is_solved(&self) -> bool {
        for i in 0..24 {
            if self.state[i] != i / 4 {
                return false;
            }
        }
        return true;
    }

    fn apply_action(&mut self, action: usize) -> Result<(), ApplyActionError> {
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
            _ => Err(ApplyActionError::InvalidActionIndex),
        }
    }

    fn get_observations(&self) -> Vec<u8> {
        let mut observations = [0; Self::OBSERVATION_SIZE];

        for i in 0..(24) {
            let value = self.state[i];
            observations[i * 6 + value] = 1;
        }

        Vec::from(observations)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn permutations_have_valid_indices() {
        for perms in PERMUTATIONS {
            for row in perms {
                for index in row {
                    assert!(index < 24);
                }
            }
        }
    }

    #[test]
    fn default_state_has_valid_values() {
        for item in DEFAULT_STATE {
            assert!(item < 6);
        }
    }

    #[test]
    fn observations_has_correct_size() {
        let cube = Cube2x2::default();
        let observations = cube.get_observations();
        assert_eq!(observations.len(), Cube2x2::OBSERVATION_SIZE);
    }

    #[test]
    fn observations_have_valid_values() {
        let cube = Cube2x2::default();
        for value in cube.get_observations() {
            assert!(value == 0 || value == 1);
        }
    }

    #[test]
    fn default_cube_is_solved() {
        let cube = Cube2x2::default();
        assert!(cube.is_solved());
    }

    #[test]
    fn applying_move_makes_cube_unsolved() {
        for i in 0..18 {
            let mut cube = Cube2x2::default();
            cube.apply_action(i).unwrap();
            assert!(!cube.is_solved());
        }
    }

    #[test]
    fn repeat_moves_loops_to_solved() {
        for i in 0..18 {
            let mut cube = Cube2x2::default();
            cube.apply_action(i).unwrap();
            cube.apply_action(i).unwrap();
            cube.apply_action(i).unwrap();
            cube.apply_action(i).unwrap();
            assert!(cube.is_solved());
        }
    }

    #[test]
    fn reset_solved_cube_is_solved() {
        let mut cube = Cube2x2::default();
        cube.reset();
        assert!(cube.is_solved());
    }

    #[test]
    fn reset_unsolved_cube_is_solved() {
        for i in 0..18 {
            let mut cube = Cube2x2::default();
            cube.apply_action(i).unwrap();
            cube.reset();
            assert!(cube.is_solved());
        }
    }

    #[test]
    fn invalid_action_returns_error() {
        let mut cube = Cube2x2::default();
        assert!(cube.apply_action(18).is_err());
    }
}
