pub trait Sigmoid {
    fn sigmoid(self) -> Self;

    fn sigmoid_derivative(self) -> Self;
}

impl Sigmoid for f64 {
    fn sigmoid(self) -> Self {
        1f64 / (1f64 + (-self).exp())
    }

    fn sigmoid_derivative(self) -> Self {
        let sigmoid = self.sigmoid();
        sigmoid * (1f64 - sigmoid)
    }
}
