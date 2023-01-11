use rand::Rng;

/// Represents a face side of the cube.  
///
/// When used for state, the Face represents that the tile has the same color as the specified face.
///
/// When used for actions, the Face represents the specific side that is being turned. It's direction is specified in the [`Move`]
#[derive(Clone, Copy)]
pub enum Face {
    /// The right side of the cube
    R,
    /// The top (up) side of the cube
    U,
    /// The front side of the cube
    F,
    /// The left side of the cube
    L,
    /// The bottom (down) side of the cube
    D,
    /// The back side of the cube
    B,
}

impl Face {
    /// Gets a face based on the index. The order is as follows:
    ///
    /// # Key
    ///
    /// | Index | Face |
    /// |-------|------|
    /// | 0     | [`Face::U`] |
    /// | 1     | [`Face::L`] |
    /// | 2     | [`Face::F`] |
    /// | 3     | [`Face::R`] |
    /// | 4     | [`Face::B`] |
    /// | 5     | [`Face::D`] |
    ///
    /// # Parameters
    /// index: (usize) The index of faces that you want to get. This should be a number between 0 and 5, inclusive
    ///
    /// # Returns
    /// Returns a result object with the Face, if the index is within valid bounds
    ///
    /// # Errors
    /// Throws an err if the index is larger than 5
    ///
    /// # Examples
    /// ```
    /// if let Ok(face) = Face::from_index(3) {
    ///     assert!(match!(face, Face::R));
    /// }
    /// ```
    ///
    pub fn from_index(index: usize) -> Result<Face, &'static str> {
        match index {
            0 => Ok(Face::U),
            1 => Ok(Face::L),
            2 => Ok(Face::F),
            3 => Ok(Face::R),
            4 => Ok(Face::B),
            5 => Ok(Face::D),
            _ => Err("Index Out of Bounds"),
        }
    }

    /// Returns permutations that the state goes through when the move is applied to it
    fn get_perms(&self) -> [[usize; 4]; 5] {
        match self {
            Face::R => [
                [20, 2, 42, 47],
                [23, 5, 39, 50],
                [26, 8, 36, 53],
                [27, 29, 35, 33],
                [28, 32, 34, 30],
            ],
            Face::U => [
                [20, 11, 38, 29],
                [19, 10, 37, 28],
                [18, 9, 36, 27],
                [8, 6, 0, 2],
                [7, 3, 1, 5],
            ],
            Face::F => [
                [6, 27, 47, 17],
                [7, 30, 46, 14],
                [8, 33, 45, 11],
                [18, 20, 26, 24],
                [19, 23, 25, 21],
            ],
            Face::L => [
                [18, 45, 44, 0],
                [21, 48, 41, 3],
                [24, 51, 38, 6],
                [11, 17, 15, 9],
                [14, 16, 12, 10],
            ],
            Face::D => [
                [24, 33, 42, 15],
                [25, 34, 43, 16],
                [26, 35, 44, 17],
                [45, 47, 53, 51],
                [46, 50, 52, 48],
            ],
            Face::B => [
                [36, 38, 44, 42],
                [37, 41, 43, 39],
                [29, 0, 15, 53],
                [32, 1, 12, 52],
                [35, 2, 9, 51],
            ],
        }
    }
}

impl ToString for Face {
    fn to_string(&self) -> String {
        match self {
            Face::R => String::from("R"),
            Face::U => String::from("U"),
            Face::F => String::from("F"),
            Face::L => String::from("L"),
            Face::D => String::from("D"),
            Face::B => String::from("B"),
        }
    }
}

impl PartialEq for Face {
    fn eq(&self, other: &Self) -> bool {
        match self {
            Face::R => match other {
                Face::R => true,
                _ => false,
            },
            Face::U => match other {
                Face::U => true,
                _ => false,
            },
            Face::F => match other {
                Face::F => true,
                _ => false,
            },
            Face::L => match other {
                Face::L => true,
                _ => false,
            },
            Face::D => match other {
                Face::D => true,
                _ => false,
            },
            Face::B => match other {
                Face::B => true,
                _ => false,
            },
        }
    }
}

pub enum Move {
    Normal(Face),
    Prime(Face),
    Two(Face),
}

impl Move {
    fn from_index(index: isize) -> Result<Move, &'static str> {
        let face = Face::from_index(index as usize / 3)?;
        match index % 3 {
            0 => Ok(Move::Normal(face)),
            1 => Ok(Move::Prime(face)),
            2 => Ok(Move::Two(face)),
            _ => Err("How did you manage to get here?!"),
        }
    }
}

impl ToString for Move {
    fn to_string(&self) -> String {
        match self {
            Move::Normal(f) => format!("{}", f.to_string()),
            Move::Prime(f) => format!("{}'", f.to_string()),
            Move::Two(f) => format!("{}2", f.to_string()),
        }
    }
}

impl PartialEq for Move {
    fn eq(&self, other: &Self) -> bool {
        match self {
            Move::Normal(f1) => match other {
                Move::Normal(f2) => f1.eq(f2),
                _ => false,
            },
            Move::Prime(f1) => match other {
                Move::Prime(f2) => f1.eq(f2),
                _ => false,
            },
            Move::Two(f1) => match other {
                Move::Two(f2) => f1.eq(f2),
                _ => false,
            },
        }
    }
}

struct Cube {
    state: [Face; 9 * 6],
}

impl Cube {
    fn new() -> Cube {
        let mut state = [Face::U; 9 * 6];
        for i in 1..6 {
            for j in 0..9 {
                state[i * 9 + j] = Face::from_index(i).unwrap();
            }
        }

        Cube { state }
    }

    fn is_solved(&self) -> bool {
        for i in 0..6 {
            let face = Face::from_index(i).unwrap();
            for j in 0..9 {
                if self.state[i * 9 + j].ne(&face) {
                    return false;
                }
            }
        }
        true
    }

    fn apply_move(&mut self, action: Move) {
        match action {
            Move::Normal(face) => {
                for row in face.get_perms() {
                    let aside = self.state[row[row.len() - 1]];
                    let len = row.len();
                    for i in 0..(len - 1) {
                        self.state[row[len - i - 1]] = self.state[row[len - i - 2]];
                    }
                    self.state[row[0]] = aside;
                }
            }
            Move::Prime(face) => {
                for row in face.get_perms() {
                    let aside = self.state[row[0]];
                    for i in 0..(row.len() - 1) {
                        self.state[row[i]] = self.state[row[i + 1]];
                    }
                    self.state[row[row.len() - 1]] = aside;
                }
            }
            Move::Two(face) => {
                for row in face.get_perms() {
                    for i in [0, 1] {
                        let aside = self.state[row[i]];
                        self.state[row[i]] = self.state[row[i + 2]];
                        self.state[row[i + 2]] = aside;
                    }
                }
            }
        }
    }


    fn scramble(&mut self, scramble_count: usize) {
        let mut rng = rand::thread_rng();
        for _ in 0..scramble_count {
            if let Ok(action) = Move::from_index(rng.gen_range(0..18)) {
                self.apply_move(action);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_all_moves() -> Vec<Move> {
        let mut moves: Vec<Move> = Vec::new();

        for face in [Face::R, Face::U, Face::F, Face::L, Face::B, Face::D] {
            moves.push(Move::Normal(face));
            moves.push(Move::Prime(face));
            moves.push(Move::Two(face));
        }

        moves
    }

    mod enum_face {
        use super::*;

        #[test]
        fn same_faces_are_equal() {
            assert!(
                Face::R.eq(&Face::R),
                "The same face should be equal to itself"
            );
        }

        #[test]
        fn different_faces_are_not_equal() {
            assert!(!Face::F.eq(&Face::L), "Different faces should not be equal");
        }

        #[test]
        fn from_index_returns_a_face() {
            assert!(match Face::from_index(0) {
                Ok(_) => true,
                Err(_) => false,
            });
        }

        #[test]
        fn from_index_returns_all_faces() {
            let mut values: Vec<Face> = Vec::new();
            for i in 0..6 {
                values.push(Face::from_index(i).unwrap());
            }
            assert!(
                values.contains(&Face::R),
                "R should be accessible using the from_index method"
            );
            assert!(
                values.contains(&Face::U),
                "U should be accessible using the from_index method"
            );
            assert!(
                values.contains(&Face::F),
                "F should be accessible using the from_index method"
            );
            assert!(
                values.contains(&Face::L),
                "L should be accessible using the from_index method"
            );
            assert!(
                values.contains(&Face::D),
                "D should be accessible using the from_index method"
            );
            assert!(
                values.contains(&Face::B),
                "B should be accessible using the from_index method"
            );
        }
    }

    mod enum_move {
        use super::*;

        #[test]
        fn same_moves_are_equal() {
            assert!(
                Move::Normal(Face::R).eq(&Move::Normal(Face::R)),
                "Moves of the same type and face should be equal"
            );
        }

        #[test]
        fn different_moves_are_not_equal() {
            assert!(
                !Move::Prime(Face::U).eq(&Move::Two(Face::U)),
                "Moves of different types but the same face should not be equal"
            );
            assert!(
                !Move::Normal(Face::F).eq(&Move::Prime(Face::L)),
                "Moves of different types and different faces should not be equal"
            );
            assert!(
                !Move::Two(Face::D).eq(&Move::Two(Face::B)),
                "Moves of the same type but different faces should not be equal"
            );
        }
    }

    mod cube {
        use super::*;

        #[test]
        fn new_cube_has_equal_face_frequencies() {
            let mut counts = [0; 6];

            let cube = Cube::new();
            for face in cube.state {
                counts[match face {
                    Face::R => 0,
                    Face::U => 1,
                    Face::F => 2,
                    Face::D => 3,
                    Face::L => 4,
                    Face::B => 5,
                }] += 1;
            }

            assert_eq!(counts[0], 9, "Frequency of Right Faces should be 9");
            assert_eq!(counts[1], 9, "Frequency of Top Faces should be 9");
            assert_eq!(counts[2], 9, "Frequency of Front Faces should be 9");
            assert_eq!(counts[3], 9, "Frequency of Bottom Faces should be 9");
            assert_eq!(counts[4], 9, "Frequency of Left Faces should be 9");
            assert_eq!(counts[5], 9, "Frequency of Back Faces should be 9");
        }

        #[test]
        fn new_cube_is_solved() {
            let cube = Cube::new();

            assert!(cube.is_solved());
        }

        #[test]
        fn applying_move_changes_state() {
            for action in get_all_moves() {
                let mut cube = Cube::new();
                let string = action.to_string();
                cube.apply_move(action);
                assert!(
                    !cube.is_solved(),
                    "Applying {} to a solved cube should make it unsolved",
                    string
                );
            }
        }

        #[test]
        fn applying_repeated_move_should_return_to_solved() {
            for i in 0..18 {
                let mut cube = Cube::new();
                for _ in 0..4 {
                    cube.apply_move(Move::from_index(i).unwrap());
                }
                assert!(
                    cube.is_solved(),
                    "Applying {} four times should return a solved cube to a solved state",
                    Move::from_index(i).unwrap().to_string()
                );
            }
        }
    }
}
