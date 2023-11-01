fn sigmoid(x: f64) -> f64 {
    (1.0) / (1.0 + (-x).exp())
}

pub struct Layer {
    input: usize,
    output: usize,
    weights: Vec<Vec<f64>>,
    bias: Vec<f64>,
}

impl Layer {
    fn new(input: usize, output: usize) -> Self {
        Self {
            input,
            output,
            weights: vec![vec![0f64; output]; input],
            bias: vec![0f64; output],
        }
    }

    fn feed_forward(&self, input: Vec<f64>) -> Result<Vec<f64>, InvalidDimension> {
        if input.len() == self.input {
            let mut values = vec![0f64; self.output];
            for j in 0..self.output {
                for i in 0..self.input {
                    values[j] += input[i] * self.weights[i][j];
                }
                values[j] = sigmoid(values[j] + self.bias[j]);
            }
            Ok(values)
        } else {
            Err(InvalidDimension {
                field: String::from("Input"),
                found: input.len(),
                expected: self.input,
            })
        }
    }
}

pub struct Network {
    layers: Vec<Layer>,
}

impl Network {
    pub fn new(input: usize, hidden_layers: Vec<usize>, output: usize) -> Self {
        let mut layers = Vec::new();
        layers.push(Layer::new(input, *hidden_layers.get(0).unwrap_or(&output)));
        if hidden_layers.len() > 0 {
            for sizes in hidden_layers.windows(2) {
                layers.push(Layer::new(sizes[0], sizes[1]));
            }
            layers.push(Layer::new(*hidden_layers.last().unwrap(), output));
        }

        Self { layers }
    }

    pub fn feed_forward(&self, input: Vec<f64>) -> Result<Vec<f64>, InvalidDimension> {
        let mut values = input;
        for layer in &self.layers {
            values = layer.feed_forward(values)?;
        }
        Ok(values)
    }
}

#[derive(Debug)]
pub struct InvalidDimension {
    pub field: String,
    pub found: usize,
    pub expected: usize,
}
