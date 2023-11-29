use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub enum ParamFunction {
    Const(f64),
    Epoch,
    UpdateInterval,
    Modular {
        base: Box<ParamFunction>,
        modular: Box<ParamFunction>,
    },
    Sum(Vec<ParamFunction>),
    Product(Vec<ParamFunction>),
    Negative(Box<ParamFunction>),
    Inverse(Box<ParamFunction>),
    Powf {
        base: Box<ParamFunction>,
        exp: Box<ParamFunction>,
    },
}

impl ParamFunction {
    pub fn calculate(&self, variables: &FunctionVariables) -> f64 {
        match self {
            Self::Const(val) => *val,
            Self::Epoch => variables.epoch as f64,
            Self::UpdateInterval => variables.update_interval as f64,
            Self::Modular { base, modular } => {
                base.calculate(variables) % modular.calculate(variables)
            }
            Self::Sum(vals) => vals.into_iter().map(|val| val.calculate(variables)).sum(),
            Self::Negative(fun) => -1f64 * fun.calculate(variables),
            Self::Product(vals) => vals
                .into_iter()
                .map(|val| val.calculate(variables))
                .product(),
            Self::Inverse(val) => 1f64 / val.calculate(variables),
            Self::Powf { base, exp } => base.calculate(variables).powf(exp.calculate(variables)),
        }
    }
}

pub struct FunctionVariables {
    pub epoch: usize,
    pub update_interval: usize,
}

/// Fucntions to simplify the Box values
impl ParamFunction {
    pub fn powf(base: ParamFunction, exp: ParamFunction) -> Self {
        Self::Powf {
            base: base.into(),
            exp: exp.into(),
        }
    }

    pub fn inverse(val: ParamFunction) -> Self {
        Self::Inverse(val.into())
    }

    pub fn negative(val: ParamFunction) -> Self {
        Self::Negative(val.into())
    }

    pub fn modular(base: ParamFunction, modular: ParamFunction) -> Self {
        Self::Modular {
            base: base.into(),
            modular: modular.into(),
        }
    }
}
