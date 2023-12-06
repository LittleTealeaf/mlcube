use std::ops::{Add, Div, Mul, Neg, Rem, Sub};

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub enum FnValue {
    Const(f64),
    Epoch,
    LastTargetUpdate,
    TargetUpdateCount,
    Add(Box<FnValue>, Box<FnValue>),
    Sub(Box<FnValue>, Box<FnValue>),
    Mul(Box<FnValue>, Box<FnValue>),
    Div(Box<FnValue>, Box<FnValue>),
    Exponent(Box<FnValue>, Box<FnValue>),
    Floor(Box<FnValue>),
    Rem(Box<FnValue>, Box<FnValue>),
    Neg(Box<FnValue>),
    Max(Box<FnValue>, Box<FnValue>),
    Min(Box<FnValue>, Box<FnValue>),
}

impl FnValue {
    pub fn calculate(&self, variables: &FunctionVariables) -> f64 {
        match self {
            FnValue::Max(a, b) => a.calculate(variables).max(b.calculate(variables)),
            FnValue::Min(a, b) => a.calculate(variables).min(b.calculate(variables)),
            FnValue::Const(val) => *val,
            FnValue::Epoch => variables.epoch as f64,
            FnValue::LastTargetUpdate => variables.last_target_update as f64,
            FnValue::TargetUpdateCount => variables.target_update_count as f64,
            FnValue::Add(a, b) => a.calculate(variables) + b.calculate(variables),
            FnValue::Sub(a, b) => a.calculate(variables) - b.calculate(variables),
            FnValue::Mul(a, b) => a.calculate(variables) * b.calculate(variables),
            FnValue::Div(a, b) => a.calculate(variables) / b.calculate(variables),
            FnValue::Exponent(a, b) => a.calculate(variables).powf(b.calculate(variables)),
            FnValue::Rem(a, b) => a.calculate(variables) % b.calculate(variables),
            FnValue::Neg(a) => a.calculate(variables) * -1f64,
            FnValue::Floor(a) => a.calculate(variables).floor(),
        }
    }
}

pub struct FunctionVariables {
    pub epoch: usize,
    pub last_target_update: usize,
    pub target_update_count: usize,
}

impl FnValue {
    pub fn exp(self, exp: Self) -> Self {
        Self::Exponent(self.into(), exp.into())
    }

    pub fn floor(self) -> Self {
        Self::Floor(self.into())
    }
}

impl From<f64> for FnValue {
    fn from(value: f64) -> Self {
        Self::Const(value)
    }
}

impl Add for FnValue {
    type Output = FnValue;
    fn add(self, rhs: Self) -> Self::Output {
        Self::Add(self.into(), rhs.into())
    }
}

impl Sub for FnValue {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::Sub(self.into(), rhs.into())
    }
}

impl Mul for FnValue {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        Self::Mul(self.into(), rhs.into())
    }
}

impl Div for FnValue {
    type Output = Self;
    fn div(self, rhs: Self) -> Self::Output {
        Self::Div(self.into(), rhs.into())
    }
}

impl Rem for FnValue {
    type Output = Self;
    fn rem(self, rhs: Self) -> Self::Output {
        Self::Rem(self.into(), rhs.into())
    }
}

impl Neg for FnValue {
    type Output = Self;
    fn neg(self) -> Self::Output {
        Self::Neg(self.into())
    }
}
