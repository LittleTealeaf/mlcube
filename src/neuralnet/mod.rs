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

    pub fn back_propagate(
        &mut self,
        inputs: Vec<f64>,
        outputs: Vec<f64>,
        alpha: f64,
    ) -> Result<(), InvalidDimension> {
        let mut layers = Vec::with_capacity(self.layers.len());
        let mut values = inputs;
        for layer in self.layers.iter_mut() {
            let output = layer.feed_forward(values.clone())?;
            layers.push((values, output.clone(), layer));
            values = output;
        }

        let mut errors = outputs.clone();
        for i in 0..errors.len() {
            errors[i] -= values[i];
        }

        for (activations, output, layer) in layers.into_iter().rev() {
            let mut new_errors = vec![0f64; layer.input];
            for j in 0..layer.output {
                let error = output[j] * (1f64 - output[j]) * errors[j];
                for i in 0..layer.input {
                    new_errors[i] += error * layer.weights[i][j];
                    layer.weights[i][j] += alpha * activations[i] * error;
                }
                layer.bias[j] += alpha * error;
            }
            errors = new_errors;
        }

        Ok(())
    }
}

#[derive(Debug)]
pub struct InvalidDimension {
    pub field: String,
    pub found: usize,
    pub expected: usize,
}
