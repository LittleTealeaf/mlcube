pub enum Rotation {
    Normal,
    Prime,
    Two,
}

pub struct InvalidRotationIndex;

impl TryFrom<usize> for Rotation {
    type Error = InvalidRotationIndex;

    fn try_from(value: usize) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Normal),
            1 => Ok(Self::Prime),
            2 => Ok(Self::Two),
            _ => Err(InvalidRotationIndex),
        }
    }
}

impl From<Rotation> for usize {
    fn from(val: Rotation) -> Self {
        match val {
            Rotation::Normal => 0,
            Rotation::Prime => 1,
            Rotation::Two => 2,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn to_index_correct() {
        assert_eq!(usize::from(Rotation::Normal), 0);
        assert_eq!(usize::from(Rotation::Prime), 1);
        assert_eq!(usize::from(Rotation::Two), 2);
    }

    #[test]
    fn from_index_returns_rotation() {
        for i in 0..3 {
            let face = Rotation::try_from(i);
            assert!(match face {
                Ok(_) => true,
                Err(_) => false,
            });
        }
    }

    #[test]
    fn from_index_is_correct() {
        for i in 0..3 {
            let face = Rotation::try_from(i);
            if let Ok(face) = face {
                let index = usize::from(face);
                assert_eq!(i, index);
            }
        }
    }

    #[test]
    fn from_invalid_index_returns_error() {
        let result = Rotation::try_from(4);
        assert!(match result {
            Ok(_) => false,
            Err(_) => true,
        })
    }
}
