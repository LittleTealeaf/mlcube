use crate::puzzle::{InvalidActionIndex, Puzzle};

const DEFAULT_STATE: [usize; 9 * 6] = [
    0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 2, 2, 2, 2, 2, 2, 2, 2, 2, 3, 3, 3, 3, 3,
    3, 3, 3, 3, 4, 4, 4, 4, 4, 4, 4, 4, 4, 5, 5, 5, 5, 5, 5, 5, 5, 5,
];

const PERMUTATIONS: [[[usize; 4]; 5]; 6] = [
    // U
    [
        [20, 11, 38, 29],
        [19, 10, 37, 28],
        [18, 9, 36, 27],
        [8, 6, 0, 2],
        [7, 3, 1, 5],
    ],
    //L
    [
        [18, 45, 44, 0],
        [21, 48, 41, 3],
        [24, 51, 38, 6],
        [11, 17, 15, 9],
        [14, 16, 12, 10],
    ],
    //F
    [
        [6, 27, 47, 17],
        [7, 30, 46, 14],
        [8, 33, 45, 11],
        [18, 20, 26, 24],
        [19, 23, 25, 21],
    ],
    // R
    [
        [20, 2, 42, 47],
        [23, 5, 39, 50],
        [26, 8, 36, 53],
        [27, 29, 35, 33],
        [28, 32, 34, 30],
    ],
    // D
    [
        [24, 33, 42, 15],
        [25, 34, 43, 16],
        [26, 35, 44, 17],
        [45, 47, 53, 51],
        [46, 50, 52, 48],
    ],
    //B
    [
        [36, 38, 44, 42],
        [37, 41, 43, 39],
        [29, 0, 15, 53],
        [32, 1, 12, 52],
        [35, 2, 9, 51],
    ],
];

pub struct Cube3x3 {
    state: [usize; 54],
}

impl Default for Cube3x3 {
    fn default() -> Self {
        Self {
            state: DEFAULT_STATE,
        }
    }
}

impl Puzzle for Cube3x3 {
    const ACTION_SIZE: usize = 18;
    const OBSERVATION_SIZE: usize = 9 * 6 * 6;

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
        let mut observations = [0; Self::OBSERVATION_SIZE];

        for i in 0..(54) {
            let value = self.state[i];
            observations[i * 6 + value] = 1;
        }

        Vec::from(observations)
    }

    fn is_solved(&self) -> bool {
        for i in 0..54 {
            if self.state[i] != i / 9 {
                return false;
            }
        }
        return true;
    }

    fn reset(&mut self) {
        self.state = DEFAULT_STATE;
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
                  assert!(index < 54);
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
      let cube = Cube3x3::default();
      let observations = cube.get_observations();
      assert_eq!(observations.len(), Cube3x3::OBSERVATION_SIZE);
  }

  #[test]
  fn observations_have_valid_values() {
      let cube = Cube3x3::default();
      for value in cube.get_observations() {
          assert!(value == 0 || value == 1);
      }
  }

  #[test]
  fn default_cube_is_solved() {
      let cube = Cube3x3::default();
      assert!(cube.is_solved());
  }

  #[test]
  fn applying_move_makes_cube_unsolved() {
      for i in 0..18 {
          let mut cube = Cube3x3::default();
          cube.apply_action(i).unwrap();
          assert!(!cube.is_solved());
      }
  }

  #[test]
  fn repeat_moves_loops_to_solved() {
      for i in 0..18 {
          let mut cube = Cube3x3::default();
          cube.apply_action(i).unwrap();
          cube.apply_action(i).unwrap();
          cube.apply_action(i).unwrap();
          cube.apply_action(i).unwrap();
          assert!(cube.is_solved());
      }
  }

  #[test]
  fn reset_solved_cube_is_solved() {
      let mut cube = Cube3x3::default();
      cube.reset();
      assert!(cube.is_solved());
  }

  #[test]
  fn reset_unsolved_cube_is_solved() {
      for i in 0..18 {
          let mut cube = Cube3x3::default();
          cube.apply_action(i).unwrap();
          cube.reset();
          assert!(cube.is_solved());
      }
  }

  #[test]
  fn invalid_action_returns_error() {
      let mut cube = Cube3x3::default();
      assert!(cube.apply_action(18).is_err());
  }
}
