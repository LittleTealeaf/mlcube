/// Represents a face side of the cube.
///
/// When used for representing state, this is used in place of the color that the given side would have. This makes it simpler and more memory efficient to calculate
#[derive(Clone, Copy)]
#[repr(u8)]
pub enum Face {
    /// The top face of the cube
    U,
    /// The left face of the cube
    L,
    /// The front face of the cube
    F,
    /// The right face of the cube
    R,
    /// The back face of the cube
    B,
    /// The bottom face of the cube
    D,
}

impl Face {
    /// Gets the face at the provided index
    ///
    /// # Parameters
    /// index
    ///     The index to get the Face at
    ///
    /// # Returns
    /// Returns a `Some(Face)` value if the index is a valid index. Returns `None` if the index is out of bounds
    /// # Index Key
    /// The resulting face is based on the following table
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
    pub fn from_index(index: u8) -> Option<Face> {
        match index {
            0 => Some(Face::U),
            1 => Some(Face::L),
            2 => Some(Face::F),
            3 => Some(Face::R),
            4 => Some(Face::B),
            5 => Some(Face::D),
            _ => None,
        }
    }

    /// Returns all possible values of the Face enum
    pub fn values() -> [Face; 6] {
        [Face::U, Face::L, Face::F, Face::R, Face::B, Face::D]
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

#[cfg(test)]
mod tests {
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
    fn values_contains_all_faces() {
        let values = Face::values();

        for face in [Face::U, Face::L, Face::F, Face::R, Face::B, Face::D] {
            assert!(
                values.contains(&face),
                "Face::values() should contain {}",
                face.to_string()
            );
        }
    }

    #[test]
    fn from_index_returns_face() {
        for i in 0..=5 {
            assert!(match Face::from_index(i) {
                Some(_) => true,
                None => false,
            });
        }
    }

    #[test]
    fn from_index_returns_none() {
        let invalid_values = [6];
        for i in invalid_values {
            assert!(match Face::from_index(i) {
                None => true,
                Some(_) => false,
            });
        }
    }

    #[test]
    fn from_index_returns_correct_faces() {
        let correct_values = [
            (0, Face::U),
            (1, Face::L),
            (2, Face::F),
            (3, Face::R),
            (4, Face::B),
            (5, Face::D)
        ];

        for (index, expected) in correct_values {
            assert!(Face::from_index(index).unwrap().eq(&expected));
        }
    }
}
