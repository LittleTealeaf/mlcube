use std::fmt::Debug;

const PERMS_R: [[usize; 4]; 5] = [
    [20, 2, 42, 47],
    [23, 5, 39, 50],
    [26, 8, 36, 53],
    [27, 29, 35, 33],
    [28, 32, 34, 30],
];

const PERMS_U: [[usize; 4]; 5] = [
    [20, 11, 38, 29],
    [19, 10, 37, 28],
    [18, 9, 36, 27],
    [8, 6, 0, 2],
    [7, 3, 1, 5],
];

const PERMS_L: [[usize; 4]; 5] = [
    [18, 45, 44, 0],
    [21, 48, 41, 3],
    [24, 51, 38, 6],
    [11, 17, 15, 9],
    [14, 16, 12, 10],
];

const PERMS_D: [[usize; 4]; 5] = [
    [24, 33, 42, 15],
    [25, 34, 43, 16],
    [26, 35, 44, 17],
    [45, 47, 53, 51],
    [46, 50, 52, 48],
];

const PERMS_F: [[usize; 4]; 5] = [
    [6, 27, 47, 17],
    [7, 30, 46, 14],
    [8, 33, 45, 11],
    [18, 20, 26, 24],
    [19, 23, 25, 21],
];

const PERMS_B: [[usize; 4]; 5] = [
    [36, 38, 44, 42],
    [37, 41, 43, 39],
    [29, 0, 15, 53],
    [32, 1, 12, 52],
    [35, 2, 9, 51],
];

#[derive(Debug)]
pub enum MoveType {
    Normal,
    Prime,
    Two,
}

impl PartialEq for MoveType {
    fn eq(&self, other: &Self) -> bool {
        match self {
            MoveType::Normal => match other {
                MoveType::Normal => true,
                _ => false,
            },
            MoveType::Prime => match other {
                MoveType::Prime => true,
                _ => false,
            },
            MoveType::Two => match other {
                MoveType::Two => true,
                _ => false,
            },
        }
    }

    fn ne(&self, other: &Self) -> bool {
        return !self.eq(other);
    }
}

impl MoveType {
    fn from_index(index: usize) -> MoveType {
        match index {
            1 => MoveType::Prime,
            2 => MoveType::Two,
            _ => MoveType::Normal,
        }
    }
}

pub enum Move {
    R(MoveType),
    U(MoveType),
    L(MoveType),
    D(MoveType),
    F(MoveType),
    B(MoveType),
}

impl Move {
    fn from_index(index: usize, modifier: MoveType) -> Move {
        match index {
            1 => Move::U(modifier),
            2 => Move::L(modifier),
            3 => Move::D(modifier),
            4 => Move::F(modifier),
            5 => Move::B(modifier),
            _ => Move::R(modifier),
        }
    }

    fn get(index: usize) -> Move {
        Move::from_index(index / 3, MoveType::from_index(index % 3))
    }
}

impl PartialEq for Move {
    fn eq(&self, other: &Self) -> bool {
        match self {
            Move::R(a) => match other {
                Move::R(b) => a.eq(b),
                _ => false,
            },
            Move::U(a) => match other {
                Move::U(b) => a.eq(b),
                _ => false,
            },
            Move::F(a) => match other {
                Move::F(b) => a.eq(b),
                _ => false,
            },
            Move::L(a) => match other {
                Move::L(b) => a.eq(b),
                _ => false,
            },
            Move::D(a) => match other {
                Move::D(b) => a.eq(b),
                _ => false,
            },
            Move::B(a) => match other {
                Move::B(b) => a.eq(b),
                _ => false,
            },
        }
    }

    fn ne(&self, other: &Self) -> bool {
        !self.eq(other)
    }
}

pub struct Cube {
    state: Vec<u8>,
}

impl Cube {
    pub fn new() -> Cube {
        let mut state: Vec<u8> = Vec::new();
        for c in 0..6 {
            for _ in 0..9 {
                state.push(c);
            }
        }

        Cube { state }
    }

    pub fn is_solved(&self) -> bool {
        let state = &self.state;
        for i in 0..state.len() {
            if state[i] != (i / 9) as u8 {
                return false;
            }
        }
        true
    }

    pub fn apply(&mut self, action: Move) {
        let (perms, modifier) = match action {
            Move::R(m) => (PERMS_R, m),
            Move::U(m) => (PERMS_U, m),
            Move::F(m) => (PERMS_F, m),
            Move::L(m) => (PERMS_L, m),
            Move::D(m) => (PERMS_D, m),
            Move::B(m) => (PERMS_B, m),
        };

        match modifier {
            MoveType::Normal => {
                for row in perms {
                    let carry = self.state[row[0]];
                    for i in 0..(row.len() - 1) {
                        self.state[row[i]] = self.state[row[i + 1]];
                    }
                    self.state[row[row.len() - 1]] = carry;
                }
            }
            MoveType::Prime => {
                for row in perms {
                    let carry = self.state[row[row.len() - 1]];
                    for i in 1..row.len() {
                        self.state[row[i]] = self.state[row[i - 1]];
                    }
                    self.state[row[0]] = carry;
                }
            }
            MoveType::Two => {
                for row in perms {
                    let carry_a = self.state[row[row.len() - 1]];
                    let carry_b = self.state[row[row.len() - 2]];
                    for i in 2..row.len() {
                        self.state[row[i]] = self.state[row[i - 2]];
                    }
                    self.state[0] = carry_b;
                    self.state[1] = carry_a;
                }
            }
        }
    }

    pub fn scramble(&mut self) {}
}

mod tests {
    use super::*;

    mod move_enum {
        use std::matches;

        use super::super::{Move, MoveType};

        #[test]
        fn from_index_returns_all_moves() {
            let mut values: Vec<Move> = Vec::new();
            for i in 0..6 {
                values.push(Move::from_index(i, MoveType::Normal));
            }

            assert!(values.contains(&Move::R(MoveType::Normal)));
            assert!(values.contains(&Move::U(MoveType::Normal)));
            assert!(values.contains(&Move::F(MoveType::Normal)));
            assert!(values.contains(&Move::D(MoveType::Normal)));
            assert!(values.contains(&Move::B(MoveType::Normal)));
            assert!(values.contains(&Move::L(MoveType::Normal)));
        }

        #[test]
        fn get_returns_all_values() {
            let mut values: Vec<Move> = Vec::new();
            for i in 0..18 {
                values.push(Move::get(i));
            }

            for i in 0..3 {
                assert!(values.contains(&Move::R(MoveType::from_index(i))));
                assert!(values.contains(&Move::U(MoveType::from_index(i))));
                assert!(values.contains(&Move::F(MoveType::from_index(i))));
                assert!(values.contains(&Move::D(MoveType::from_index(i))));
                assert!(values.contains(&Move::B(MoveType::from_index(i))));
                assert!(values.contains(&Move::L(MoveType::from_index(i))));
            }
        }

        #[test]
        fn same_moves_are_equal() {
            assert!(Move::R(MoveType::Normal).eq(&Move::R(MoveType::Normal)));
            assert!(!Move::R(MoveType::Normal).ne(&Move::R(MoveType::Normal)));
        }

        #[test]
        fn different_moves_are_not_equal() {
            assert!(!Move::R(MoveType::Normal).eq(&Move::L(MoveType::Normal)));
            assert!(!Move::R(MoveType::Normal).eq(&Move::R(MoveType::Two)));
            assert!(Move::R(MoveType::Normal).ne(&Move::L(MoveType::Normal)));
            assert!(Move::R(MoveType::Normal).ne(&Move::R(MoveType::Two)));
        }
    }

    mod move_type_enum {
        use std::matches;

        use super::super::MoveType;

        #[test]
        fn from_index_returns_all_types() {
            let mut values: Vec<MoveType> = Vec::new();
            for i in 0..3 {
                values.push(MoveType::from_index(i));
            }

            assert!(values.contains(&MoveType::Normal));
            assert!(values.contains(&MoveType::Two));
            assert!(values.contains(&MoveType::Prime));
        }

        #[test]
        fn same_types_are_equal() {
            assert!(MoveType::Normal.eq(&MoveType::Normal));
            assert!(!MoveType::Normal.ne(&MoveType::Normal));
        }

        #[test]
        fn different_types_are_not_equal() {
            assert!(!MoveType::Normal.eq(&MoveType::Two));
            assert!(MoveType::Normal.ne(&MoveType::Two));
        }
    }

    mod cube {

        use super::super::Cube;

        #[test]
        fn new_cube_is_solved() {
            let env = Cube::new();
            assert!(env.is_solved());
        }
    }
}
