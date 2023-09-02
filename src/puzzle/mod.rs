pub trait Puzzle {
    const STATE_LENGTH: usize;
    type Action: Action;

    fn apply_action(&mut self, action: Self::Action);
}

pub trait Action: From<usize> {
    const COUNT: usize;
}


pub trait State {

}
