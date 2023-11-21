use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub enum Function {
    Const(f64),
    Variable,
    Add(Box<Self>, Box<Self>),
    Multiple(Box<Self>, Box<Self>),
    Subtract(Box<Self>, Box<Self>),
    Divide(Box<Self>, Box<Self>),
}

impl Function {
    pub fn calculate(&self, var: f64) -> f64 {
        match self {
            Self::Const(f) => *f,
            Self::Variable => var,
            Self::Add(a, b) => a.calculate(var) + b.calculate(var),
            Self::Multiple(a, b) => a.calculate(var) * b.calculate(var),
            Self::Subtract(a, b) => a.calculate(var) - b.calculate(var),
            Self::Divide(a, b) => a.calculate(var) / b.calculate(var),
        }
    }
}
