

pub trait Environment: Clone {
    fn new() -> Self;

    fn apply(self, action: usize) -> Result<Self,ActionIndexOutOfBounds>;

    fn reward(&self) -> f64;
}

#[derive(Debug)]
pub struct ActionIndexOutOfBounds(usize);
