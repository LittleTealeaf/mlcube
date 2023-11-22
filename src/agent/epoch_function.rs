use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub enum EpochFunction {
    Const(f64),
    WithinTargetPow { scale: f64, base: f64 },
}

impl EpochFunction {
    pub fn calculate(&self, epoch: usize, update_interval: usize) -> f64 {
        match self {
            Self::Const(val) => *val,
            Self::WithinTargetPow { scale, base } => {
                base * scale.powi((epoch % update_interval + 1) as i32)
            }
        }
    }
}
