/// Implements Feed-Forward and Back Propagation
///
/// # Generics
/// - **IN**: `usize` Number of data nodes coming into the layer
/// - **OUT**: `usize` Number of output nodes of the entire network.
pub trait NeuralNetworkLayer<const IN: usize, const OUT: usize> {
    fn feed_forward(&self, inputs: [f64; IN]) -> [f64; OUT];

    fn back_propagate(&mut self, inputs: [f64; IN], outputs: [f64; OUT], alpha: f64) -> [f64; IN];
}

fn sigmoid(x: f64) -> f64 {
    1.0 / (1.0 + (-x).exp())
}

pub struct HiddenLayer<
    Next,
    const LAYER_IN: usize,
    const LAYER_OUT: usize,
    const NETWORK_OUT: usize,
> where
    Next: NeuralNetworkLayer<LAYER_OUT, NETWORK_OUT>,
{
    next: Next,
    weights: [[f64; LAYER_OUT]; LAYER_IN],
    biases: [f64; LAYER_OUT],
}

impl<Next, const LAYER_IN: usize, const LAYER_OUT: usize, const NETWORK_OUT: usize>
    NeuralNetworkLayer<LAYER_IN, NETWORK_OUT>
    for HiddenLayer<Next, LAYER_IN, LAYER_OUT, NETWORK_OUT>
where
    Next: NeuralNetworkLayer<LAYER_OUT, NETWORK_OUT>,
{
    fn feed_forward(&self, inputs: [f64; LAYER_IN]) -> [f64; NETWORK_OUT] {
        let mut outputs = [0f64; LAYER_OUT];
        for i in 0..LAYER_OUT {
            outputs[i] += self.biases[i];
            for j in 0..LAYER_IN {
                outputs[i] += inputs[j] * self.weights[j][i];
            }
            outputs[i] = sigmoid(outputs[i]);
        }
        self.next.feed_forward(outputs)
    }

    fn back_propagate(
        &mut self,
        inputs: [f64; LAYER_IN],
        outputs: [f64; NETWORK_OUT],
        alpha: f64,
    ) -> [f64; LAYER_IN] {
        let mut layer_out = [0f64; LAYER_OUT];
        for i in 0..LAYER_OUT {
            layer_out[i] += self.biases[i];
            for j in 0..LAYER_IN {
                layer_out[i] += inputs[j] * self.weights[j][i];
            }
            layer_out[i] = sigmoid(layer_out[i]);
        }
        let next_errors = self.next.back_propagate(layer_out, outputs, alpha);

        let mut errors = [0f64; LAYER_IN];

        for i in 0..LAYER_OUT {
            layer_out[i] = layer_out[i] * (1f64 - layer_out[i]) * next_errors[i];

            for j in 0..LAYER_IN {
                errors[j] += self.weights[j][i] * layer_out[i];
                self.weights[j][i] += alpha * inputs[j] * layer_out[i];
            }
            self.biases[i] += alpha * layer_out[i];
        }

        errors
    }
}

pub struct OutputLayer<const IN: usize, const OUT: usize> {
    weights: [[f64; OUT]; IN],
    biases: [f64; OUT],
}

impl<const LAYER_IN: usize, const LAYER_OUT: usize> NeuralNetworkLayer<LAYER_IN, LAYER_OUT>
    for OutputLayer<LAYER_IN, LAYER_OUT>
{
    fn feed_forward(&self, inputs: [f64; LAYER_IN]) -> [f64; LAYER_OUT] {
        let mut outputs = [0f64; LAYER_OUT];
        for i in 0..LAYER_OUT {
            outputs[i] += self.biases[i];
            for j in 0..LAYER_IN {
                outputs[i] += inputs[j] * self.weights[j][i];
            }
            outputs[i] = sigmoid(outputs[i]);
        }
        outputs
    }

    fn back_propagate(
        &mut self,
        inputs: [f64; LAYER_IN],
        outputs: [f64; LAYER_OUT],
        alpha: f64,
    ) -> [f64; LAYER_IN] {
        let mut layer_out = [0f64; LAYER_OUT];
        for i in 0..LAYER_OUT {
            layer_out[i] += self.biases[i];
            for j in 0..LAYER_IN {
                layer_out[i] += inputs[j] * self.weights[j][i];
            }
            layer_out[i] = sigmoid(layer_out[i]);
        }

        let mut next_errors = layer_out.clone();
        for i in 0..LAYER_OUT {
            next_errors[i] = outputs[i] - next_errors[i];
        }

        let mut errors = [0f64; LAYER_IN];

        for i in 0..LAYER_OUT {
            layer_out[i] = layer_out[i] * (1f64 - layer_out[i]) * next_errors[i];

            for j in 0..LAYER_IN {
                errors[j] += self.weights[j][i] * layer_out[i];
                self.weights[j][i] += alpha * inputs[j] * layer_out[i];
            }
            self.biases[i] += alpha * layer_out[i];
        }

        errors
    }
}
