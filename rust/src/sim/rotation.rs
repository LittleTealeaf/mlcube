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
