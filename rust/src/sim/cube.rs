use super::{Action, Face, Rotation};

pub struct Cube {
    state: [Face; 9 * 6],
}

impl Cube {
    pub fn apply_action(&mut self, action: &Action) {
        let permutations = action.get_permutations();
        match action.rotation {
            Rotation::Normal => {
                for row in permutations {
                    let tmp = self.state[row[3]];
                    for i in 0..3 {
                        self.state[row[3 - i]] = self.state[row[2 - i]];
                    }
                    self.state[row[0]] = tmp;
                }
            }
            Rotation::Prime => {
                for row in permutations {
                    let tmp = self.state[row[0]];
                    for i in 0..3 {
                        self.state[row[i]] = self.state[row[i + 1]];
                    }
                    self.state[row[3]] = tmp;
                }
            }
            Rotation::Two => {
                for row in permutations {
                    for i in [0, 1] {
                        let tmp = self.state[row[i]];
                        self.state[row[i]] = self.state[row[i + 2]];
                        self.state[row[i + 2]] = tmp;
                    }
                }
            }
        }
    }

    pub fn get_observations(&self) -> [u8; 54 * 6] {
        let mut values = [0; 54 * 6];

        for i in 0..54 {
            values[i * 6 + usize::from(self.state[i])] = 1;
        }

        values
    }

    pub fn reset(&mut self) {
        self.state = get_initial_state();
    }

    pub fn is_solved(&self) -> bool {
        for i in 0..54 {
            if usize::from(self.state[i]) != i / 9 {
                return false;
            }
        }

        true
    }
}

impl Default for Cube {
    fn default() -> Self {
        Self {
            state: get_initial_state(),
        }
    }
}

fn get_initial_state() -> [Face; 54] {
    [
        Face::U,
        Face::U,
        Face::U,
        Face::U,
        Face::U,
        Face::U,
        Face::U,
        Face::U,
        Face::U,
        Face::L,
        Face::L,
        Face::L,
        Face::L,
        Face::L,
        Face::L,
        Face::L,
        Face::L,
        Face::L,
        Face::F,
        Face::F,
        Face::F,
        Face::F,
        Face::F,
        Face::F,
        Face::F,
        Face::F,
        Face::F,
        Face::R,
        Face::R,
        Face::R,
        Face::R,
        Face::R,
        Face::R,
        Face::R,
        Face::R,
        Face::R,
        Face::B,
        Face::B,
        Face::B,
        Face::B,
        Face::B,
        Face::B,
        Face::B,
        Face::B,
        Face::B,
        Face::D,
        Face::D,
        Face::D,
        Face::D,
        Face::D,
        Face::D,
        Face::D,
        Face::D,
        Face::D,
    ]
}
