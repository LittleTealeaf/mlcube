pub enum Face {
    R,
    U,
    F,
    L,
    D,
    B,
}

pub struct InvalidFaceIndex;

impl TryFrom<usize> for Face {
    type Error = InvalidFaceIndex;

    fn try_from(value: usize) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::U),
            1 => Ok(Self::L),
            2 => Ok(Self::F),
            3 => Ok(Self::R),
            4 => Ok(Self::B),
            5 => Ok(Self::D),
            _ => Err(InvalidFaceIndex),
        }
    }
}

impl From<Face> for usize {
    fn from(val: Face) -> Self {
        match val {
            Face::U => 0,
            Face::L => 1,
            Face::F => 2,
            Face::R => 3,
            Face::B => 4,
            Face::D => 5,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn to_index_correct() {
        assert_eq!(usize::from(Face::U), 0);
        assert_eq!(usize::from(Face::L), 1);
        assert_eq!(usize::from(Face::F), 2);
        assert_eq!(usize::from(Face::R), 3);
        assert_eq!(usize::from(Face::B), 4);
        assert_eq!(usize::from(Face::D), 5);
    }

    #[test]
    fn from_index_returns_face() {
        for i in 0..6 {
            let face = Face::try_from(i);
            assert!(match face {
                Ok(_) => true,
                Err(_) => false,
            });
        }
    }

    #[test]
    fn from_index_is_correct() {
        for i in 0..6 {
            let face = Face::try_from(i);
            if let Ok(face) = face {
                let index = usize::from(face);
                assert_eq!(i, index);
            }
        }
    }

    #[test]
    fn from_invalid_index_returns_error() {
        let result = Face::try_from(6);
        assert!(match result {
            Ok(_) => false,
            Err(_) => true,
        })
    }
}
