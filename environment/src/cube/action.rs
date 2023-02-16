pub enum Action {
    Normal,Prime,Two
}

impl Action {
    pub fn from_index(index: usize) -> Option<Self> {
      match index {
        0 => Some(Self::Normal),
        1 => Some(Self::Prime),
        2 => Some(Self::Two),
        _ => None,
    }
    }
}
