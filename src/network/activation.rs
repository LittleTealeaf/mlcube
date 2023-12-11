use crate::utils::Relu;

pub trait Activation {
    fn activation(self) -> Self;
    fn activation_derivative(self) -> Self;
}

impl Activation for f64 {
    fn activation(self) -> Self {
        self.relu()
    }

    fn activation_derivative(self) -> Self {
        self.relu_derivative()
    }
}
