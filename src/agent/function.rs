use std::ops::{Add, Div, Mul, Neg, Rem, Sub};

use serde::{Deserialize, Serialize};

// Const(f64),
// Epoch,
// UpdateInterval,
// Modular {
//     base: Box<ParamFunction>,
//     modular: Box<ParamFunction>,
// },
// Sum(Vec<ParamFunction>),
// Product(Vec<ParamFunction>),
// Negative(Box<ParamFunction>),
// Inverse(Box<ParamFunction>),
// Exponent {
//     base: Box<ParamFunction>,
//     exp: Box<ParamFunction>,
// },

#[derive(Serialize, Deserialize)]
pub enum Value {
    Const(f64),
    Epoch,
    UpdateInterval,
    Add(Box<Value>, Box<Value>),
    Sub(Box<Value>, Box<Value>),
    Mul(Box<Value>, Box<Value>),
    Div(Box<Value>, Box<Value>),
    Exponent { base: Box<Value>, exp: Box<Value> },
    Rem(Box<Value>, Box<Value>),
    Neg(Box<Value>),
}

impl Value {
    pub fn calculate(&self, variables: &FunctionVariables) -> f64 {
        match self {
            Value::Const(val) => *val,
            Value::Epoch => variables.epoch as f64,
            Value::UpdateInterval => variables.update_interval as f64,
            Value::Add(a, b) => a.calculate(variables) + b.calculate(variables),
            Value::Sub(a, b) => a.calculate(variables) - b.calculate(variables),
            Value::Mul(a, b) => a.calculate(variables) * b.calculate(variables),
            Value::Div(a, b) => a.calculate(variables) / b.calculate(variables),
            Value::Exponent { base, exp } => {
                base.calculate(variables).powf(exp.calculate(variables))
            }
            Value::Rem(a, b) => a.calculate(variables) % b.calculate(variables),
            Value::Neg(a) => a.calculate(variables) * -1f64,
        }
    }
}

pub struct FunctionVariables {
    pub epoch: usize,
    pub update_interval: usize,
}

impl Value {
    pub fn exp(self, exp: Self) -> Self {
        Self::Exponent {
            base: self.into(),
            exp: exp.into(),
        }
    }
}

impl From<f64> for Value {
    fn from(value: f64) -> Self {
        Self::Const(value)
    }
}

impl Add for Value {
    type Output = Value;
    fn add(self, rhs: Self) -> Self::Output {
        Self::Add(self.into(), rhs.into())
    }
}

impl Sub for Value {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::Sub(self.into(), rhs.into())
    }
}

impl Mul for Value {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        Self::Mul(self.into(), rhs.into())
    }
}

impl Div for Value {
    type Output = Self;
    fn div(self, rhs: Self) -> Self::Output {
        Self::Div(self.into(), rhs.into())
    }
}

impl Rem for Value {
    type Output = Self;
    fn rem(self, rhs: Self) -> Self::Output {
        Self::Rem(self.into(), rhs.into())
    }
}

impl Neg for Value {
    type Output = Self;
    fn neg(self) -> Self::Output {
        Self::Neg(self.into())
    }
}
