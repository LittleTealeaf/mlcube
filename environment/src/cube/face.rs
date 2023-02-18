#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Face {
    R,
    U,
    F,
    L,
    D,
    B,
}

impl Face {
    pub fn from_index(index: usize) -> Option<Self> {
        match index {
            0 => Some(Self::U),
            1 => Some(Self::L),
            2 => Some(Self::F),
            3 => Some(Self::R),
            4 => Some(Self::B),
            5 => Some(Self::D),
            _ => None,
        }
    }

    pub fn to_index(&self) -> usize {
        match self {
            Self::U => 0,
            Self::L => 1,
            Self::F => 2,
            Self::R => 3,
            Self::B => 4,
            Self::D => 5,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_index() {
        assert_eq!(Face::from_index(0).unwrap(), Face::U);
        assert_eq!(Face::from_index(1).unwrap(), Face::L);
        assert_eq!(Face::from_index(2).unwrap(), Face::F);
        assert_eq!(Face::from_index(3).unwrap(), Face::R);
        assert_eq!(Face::from_index(4).unwrap(), Face::B);
        assert_eq!(Face::from_index(5).unwrap(), Face::D);
        assert_eq!(Face::from_index(6), None);
    }

    #[test]
    fn to_index() {
        assert_eq!(Face::U.to_index(), 0);
        assert_eq!(Face::L.to_index(), 1);
        assert_eq!(Face::F.to_index(), 2);
        assert_eq!(Face::R.to_index(), 3);
        assert_eq!(Face::B.to_index(), 4);
        assert_eq!(Face::D.to_index(), 5);
    }
}
