#[derive(Clone, Copy)]
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
}
