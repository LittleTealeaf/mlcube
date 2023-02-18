#[derive(Clone, Copy, PartialEq, Eq)]
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
    fn all_index_match_up() {
        for face in [Face::U, Face::L, Face::F, Face::R, Face::B, Face::D] {
            let index = face.to_index();
            let resolved = Face::from_index(index);
            assert!(match resolved {
                Some(resolved_face) => resolved_face.eq(&face),
                None => false,
            });
        }
    }
}
