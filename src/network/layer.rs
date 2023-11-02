pub struct Layer {
    weights: Vec<Vec<f64>>,
    bias: Vec<f64>,
}

impl Layer {
    pub fn new(input: usize, output: usize) -> Self {
        Self {
            weights: vec![vec![0f64; output]; input],
            bias: vec![0f64; output],
        }
    }

    pub fn apply(&self, input: &Vec<f64>) -> Result<Vec<f64>, InvalidInputSize> {
        if input.len() != self.weights.len() {
            Err(InvalidInputSize {
                expected: input.len(),
                found: self.weights.len(),
            })
        } else {
            let mut values = self.bias.clone();
            for i in 0..self.bias.len() {
                for j in 0..self.weights.len() {
                    values[i] += input[j] * self.weights[j][i];
                }
            }
            Ok(values)
        }
    }
}

#[derive(Debug)]
pub struct InvalidInputSize {
    expected: usize,
    found: usize,
}
