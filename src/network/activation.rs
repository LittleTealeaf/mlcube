use crate::utils::{Relu, Sigmoid};

pub trait Activation {
    fn activation(self) -> Self;
    fn activation_derivative(self) -> Self;
}

impl Activation for f64 {
    fn activation(self) -> Self {
        self.sigmoid()
    }

    fn activation_derivative(self) -> Self {
        self.sigmoid_derivative()
    }
}
