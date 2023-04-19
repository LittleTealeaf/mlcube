use crate::puzzle::{ApplyActionError, ParseActionError, Puzzle};

pub struct Cube2x2 {
    /// The current state of the 2x2 Cube
    state: [usize; 24],
}

impl Default for Cube2x2 {
    fn default() -> Self {
        Self {
            state: DEFAULT_STATE,
        }
    }
}

impl Cube2x2 {
    pub fn get_state(&self) -> [usize; 24] {
        self.state.clone()
    }
}

impl Puzzle for Cube2x2 {
    const OBSERVATION_SIZE: usize = 4 * 6 * 6;
    const ACTION_SIZE: usize = 9;

    fn apply_action(&mut self, action: usize) -> Result<(), ApplyActionError> {
        let permutations = PERMUTATIONS[action % 3];
        let rotation = action / 3;
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

    fn get_reward(&self) -> f64 {
        let mut total = 0f64;
        for i in 0..24 {
            if self.state[i] == i / 4 {
                total += 1f64;
            }
        }

        total
    }

    fn get_action_name(action: usize) -> Option<String> {
        let move_name = match action % 3 {
            0 => Some('U'),
            1 => Some('F'),
            2 => Some('R'),
            _ => None,
        };
        let move_type = match action / 3 {
            0 => Some(""),
            1 => Some("'"),
            2 => Some("2"),
            _ => None,
        };

        Some(format!("{}{}", move_name?, move_type?))
    }

    fn parse_action_name(name: &str) -> Result<usize, ParseActionError> {
        let mut chars = name.chars();
        let name = chars.next().ok_or(ParseActionError::StringParseError)?;
        let modifier = chars.next();

        let name_index = match name {
            'U' => Ok(0),
            'F' => Ok(1),
            'R' => Ok(2),
            _ => Err(ParseActionError::InvalidActionName),
        }?;

        let type_index = match modifier {
            None => Ok(0),
            Some('\'') => Ok(1),
            Some('2') => Ok(2),
            _ => Err(ParseActionError::InvalidModifier),
        }?;

        Ok(type_index * 3 + name_index)
    }
}

const DEFAULT_STATE: [usize; 24] = [
    0, 0, 0, 0, 1, 1, 1, 1, 2, 2, 2, 2, 3, 3, 3, 3, 4, 4, 4, 4, 5, 5, 5, 5,
];

const PERMUTATIONS: [[[usize; 4]; 3]; 3] = [
    // U
    [[0, 1, 3, 2], [8, 4, 16, 12], [9, 5, 17, 13]],
    // F
    [[8, 9, 11, 10], [5, 3, 14, 20], [7, 2, 12, 21]],
    // R
    [[14, 12, 13, 15], [9, 1, 18, 21], [11, 3, 16, 23]],
];

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use super::*;

    #[test]
    fn permutations_have_valid_indices() {
        for perms in PERMUTATIONS {
            for row in perms {
                for index in row {
                    assert!(
                        index < 24,
                        "Index values must be less than 24, found {}",
                        index
                    );
                }
            }
        }
    }

    #[test]
    fn default_state_has_valid_values() {
        for item in DEFAULT_STATE {
            assert!(
                item < 6,
                "A color value should be within the range [0,5], found {}",
                item
            );
        }
    }

    #[test]
    fn observations_has_correct_size() {
        let cube = Cube2x2::default();
        let observations = cube.get_observations();
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
        let cube = Cube2x2::default();
        for value in cube.get_observations() {
            assert!(
                value == 0 || value == 1,
                "Any value in cube should be 0 or 1, found {}",
                value
            );
        }
    }

    #[test]
    fn default_cube_is_solved() {
        let cube = Cube2x2::default();
        assert!(
            cube.is_solved(),
            "A default cube should be solved, found unsolved cube"
        );
    }

    #[test]
    fn applying_move_makes_cube_unsolved() {
        for i in 0..Cube2x2::ACTION_SIZE {
            let mut cube = Cube2x2::default();
            cube.apply_action(i).unwrap();

            assert!(
                !cube.is_solved(),
                "Applying the action {} should be unsolved, found a solved cube",
                i
            );
        }
    }

    #[test]
    fn repeat_moves_loops_to_solved() {
        for i in 0..Cube2x2::ACTION_SIZE {
            let mut cube = Cube2x2::default();
            cube.apply_action(i).unwrap();
            cube.apply_action(i).unwrap();
            cube.apply_action(i).unwrap();
            cube.apply_action(i).unwrap();
            assert!(
                cube.is_solved(),
                "Applying the action {} four times should be solved, found an unsolved cube",
                i
            );
        }
    }

    #[test]
    fn reset_solved_cube_is_solved() {
        let mut cube = Cube2x2::default();
        cube.reset();
        assert!(
            cube.is_solved(),
            "A cube should be solved after resetting it"
        );
    }

    #[test]
    fn reset_unsolved_cube_is_solved() {
        for i in 0..Cube2x2::ACTION_SIZE {
            let mut cube = Cube2x2::default();
            cube.apply_action(i).unwrap();
            cube.reset();
            assert!(
                cube.is_solved(),
                "A cube unsolved by the action {} should be solved after a reset",
                i
            );
        }
    }

    #[test]
    fn invalid_action_returns_error() {
        let mut cube = Cube2x2::default();
        assert!(
            cube.apply_action(Cube2x2::ACTION_SIZE).is_err(),
            "Applying the action {} should return an Err because {} is an invalid action",
            Cube2x2::ACTION_SIZE,
            Cube2x2::ACTION_SIZE
        );
    }

    #[test]
    fn observations_have_valid_format() {
        let cube = Cube2x2::default();
        let observations = cube.get_observations();
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
        let mut cube = Cube2x2::default();
        let seed = 1234;
        cube.scramble_with_seed(100, seed);
        assert!(!cube.is_solved());
    }

    #[test]
    fn scramble_unsolves_cube() {
        let mut cube = Cube2x2::default();
        cube.scramble(100);
        assert!(!cube.is_solved());
    }

    #[test]
    fn scramble_returns_correct_seed() {
        let mut cube = Cube2x2::default();
        let seed = cube.scramble(100);

        let mut cube_clone = Cube2x2::default();
        cube_clone.scramble_with_seed(100, seed);

        for i in 0..24 {
            assert_eq!(cube.state[i], cube_clone.state[i]);
        }
    }

    #[test]
    fn scramble_seeds_are_random() {
        let mut visited_seeds = Vec::new();
        for _ in 0..100 {
            let mut cube = Cube2x2::default();
            let seed = cube.scramble(10);
            assert!(
                !visited_seeds.contains(&seed),
                "Duplicate Seed Found: {}",
                seed
            );
            visited_seeds.push(seed);
        }
    }

    #[test]
    fn scramble_seeds_produce_identical_cubes() {
        let seed = 12342;

        let mut cube_a = Cube2x2::default();
        cube_a.scramble_with_seed(100, seed);

        let mut cube_b = Cube2x2::default();
        cube_b.scramble_with_seed(100, seed);

        for i in 0..24 {
            assert_eq!(cube_a.state[i], cube_b.state[i]);
        }
    }

    #[test]
    fn reward_drops_after_move() {
        let mut cube = Cube2x2::default();

        for i in 0..Cube2x2::ACTION_SIZE {
            cube.reset();
            let reward_1 = cube.get_reward();
            cube.apply_action(i).unwrap();
            let reward_2 = cube.get_reward();

            assert!(reward_1 > reward_2);
        }
    }

    #[test]
    fn scramble_cubes_have_less_reward() {
        let mut cube = Cube2x2::default();

        let reward_1 = cube.get_reward();

        cube.scramble(100);

        let reward_2 = cube.get_reward();

        assert!(reward_1 > reward_2);
    }

    #[test]
    fn action_names_are_unique() {
        let mut values = HashSet::new();

        for i in 0..Cube2x2::ACTION_SIZE {
            assert!(match Cube2x2::get_action_name(i) {
                Some(value) => {
                    assert!(!values.contains(&value));
                    values.insert(value);
                    true
                }
                None => false,
            });
        }
    }

    #[test]
    fn action_names_have_correct_value() {
        let correct_values = [
            (0, "U"),
            (1, "F"),
            (2, "R"),
            (3, "U'"),
            (4, "F'"),
            (5, "R'"),
            (6, "U2"),
            (7, "F2"),
            (8, "R2"),
        ];

        for (index, value) in correct_values {
            let found_value = Cube2x2::get_action_name(index).unwrap();
            assert_eq!(
                found_value,
                String::from(value),
                "Index {} returns {}, expected {}",
                index,
                found_value,
                value
            );
        }
    }
}
