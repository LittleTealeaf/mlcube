use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub enum EpochFunction {
    Const(f64),
    WithinTargetPow {
        base: f64,
        scale: f64,
        intercept: f64,
    },
    PerTargetPow {
        base: f64,
        scale: f64,
        intercept: f64,
    },
}

impl EpochFunction {
    pub fn calculate(&self, epoch: usize, update_interval: usize) -> f64 {
        match self {
            Self::Const(val) => *val,
            Self::WithinTargetPow {
                intercept,
                scale,
                base,
            } => intercept + base * scale.powi((epoch % update_interval) as i32),
            Self::PerTargetPow {
                base,
                scale,
                intercept,
            } => intercept + base * scale.powi((epoch / update_interval) as i32),
        }
    }
}
