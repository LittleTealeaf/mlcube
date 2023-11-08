

pub trait Puzzle: Clone {
    /// The number of actions that can be taken on the puzzle
    const ACTIONS_COUNT: usize;
    const FEATURE_LENGTH: usize;
}
