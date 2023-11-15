use rand::{rngs::ThreadRng, Rng, distributions::uniform::SampleRange};

use crate::utils::Relu;

#[derive(Clone, Debug)]
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
                outputs[j] += inputs[i] * self.weights[self.get_weights_index(i, j)];
            }
            outputs[j] = outputs[j].relu();
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

    pub fn randomize<R>(&mut self, rng: &mut ThreadRng, range: R) where R: SampleRange<f64> + Clone {
        for i in 0..self.weights.len() {
            self.weights[i] = rng.gen_range(range.clone());
        }

        for i in 0..self.bias.len() {
            self.bias[i] = rng.gen_range(range.clone());
        }
    }

    pub fn back_propagate(
        &self,
        features: Vec<f64>,
        errors: &Vec<f64>,
        outputs: &Vec<f64>,
    ) -> LayerBackPropagate {
        let mut nudge = self.copy_size();
        let mut new_errors = vec![0f64; self.inputs];
        for j in 0..self.outputs {
            let error = outputs[j].relu_derivative() * errors[j];
            for i in 0..self.inputs {
                let index = self.get_weights_index(i, j);
                new_errors[i] += error * self.weights[index];
                nudge.weights[index] += features[i] * error;
            }
            nudge.bias[j] += error;
        }
        LayerBackPropagate {
            error: new_errors,
            nudge,
        }
    }

    pub fn back_propagate_output(
        &self,
        features: Vec<f64>,
        error: f64,
        index: usize,
    ) -> LayerBackPropagate {
        let mut nudge = self.copy_size();
        let mut new_errors = vec![0f64; self.inputs];
        for i in 0..self.inputs {
            let index = self.get_weights_index(i, index);
            new_errors[i] += error * self.weights[index];
            nudge.weights[index] += features[i] * error;
        }
        nudge.bias[index] += error;

        LayerBackPropagate {
            error: new_errors,
            nudge,
        }
    }


    pub fn get_inputs(&self) -> usize {
        self.inputs
    }

    pub fn get_outputs(&self) -> usize {
        self.outputs
    }
}

pub struct LayerBackPropagate {
    pub error: Vec<f64>,
    pub nudge: Layer,
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
