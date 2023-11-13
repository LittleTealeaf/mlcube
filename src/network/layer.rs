use rand::{rngs::ThreadRng, Rng};

#[derive(Clone)]
pub struct Layer {
    weights: Vec<f64>,
    bias: Vec<f64>,
    inputs: usize,
    outputs: usize,
}

impl Layer {
    pub fn new(inputs: usize, outputs: usize) -> Self {
        Self {
            weights: vec![0f64; inputs * outputs],
            bias: vec![0f64; outputs],
            inputs,
            outputs,
        }
    }

    pub fn copy_size(&self) -> Self {
        Self::new(self.inputs, self.outputs)
    }

    pub fn apply(&self, inputs: Vec<f64>) -> Vec<f64> {
        let mut outputs = self.bias.clone();
        for j in 0..self.outputs {
            for i in 0..self.inputs {
                outputs[j] += inputs[j] * self.weights[self.get_weights_index(i, j)];
            }
            if outputs[j] < 0f64 {
                outputs[j] = 0f64;
            }
        }
        outputs
    }

    pub fn scale(&mut self, scale: f64) {
        for i in 0..self.weights.len() {
            self.weights[i] *= scale;
        }
        for i in 0..self.bias.len() {
            self.bias[i] *= scale;
        }
    }

    pub fn add(&mut self, other: Layer) {
        if self.inputs != other.inputs || self.outputs != other.outputs {
            panic!("Invalid Dimensions");
        }

        for i in 0..self.weights.len() {
            self.weights[i] += other.weights[i];
        }

        for i in 0..self.bias.len() {
            self.bias[i] += other.bias[i];
        }
    }

    pub fn randomize(&mut self, rng: &mut ThreadRng) {
        for i in 0..self.weights.len() {
            self.weights[i] = rng.gen_range(-1f64..1f64);
        }

        for i in 0..self.bias.len() {
            self.bias[i] = rng.gen_range(-1f64..1f64);
        }
    }
}

impl Layer {
    #[inline(always)]
    fn get_weights_index(&self, input: usize, output: usize) -> usize {
        input * self.outputs + output
    }

    #[inline(always)]
    fn get_weight_coordinates(&self, index: usize) -> (usize, usize) {
        (index / self.inputs, index % self.inputs)
    }
}
