pub struct Layer {
    weights: Vec<f64>,
    bias: Vec<f64>,
    input_length: usize,
    output_length: usize,
}

impl Layer {
    pub fn new(input_length: usize, output_length: usize) -> Self {
        Self {
            weights: vec![0f64; input_length * output_length],
            bias: vec![0f64; output_length],
            input_length,
            output_length,
        }
    }
}

impl Layer {
    #[inline(always)]
    fn get_weight_index(&self, input: usize, output: usize) -> usize {
        input * self.input_length + output
    }

    #[inline(always)]
    fn get_weight_coordinates(&self, index: usize) -> (usize, usize) {
        (index / self.input_length, index % self.input_length)
    }
}
