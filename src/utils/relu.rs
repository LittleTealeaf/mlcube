pub trait Relu {
    fn relu(self) -> Self;
    fn relu_derivative(self) -> Self;
}

impl Relu for f64 {
    fn relu(self) -> Self {
        self.max(0f64)
    }

    fn relu_derivative(self) -> Self {
        if self >= 0f64 {
            1f64
        } else {
            0f64
        }
    }
}
