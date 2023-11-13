mod layer;
use std::marker::PhantomData;

pub use layer::*;
use rand::rngs::ThreadRng;

use crate::puzzle::Puzzle;

pub struct Network<P>
where
    P: Puzzle,
{
    layers: Vec<Layer>,
    _puzzle: PhantomData<P>,
}

impl<P> Network<P>
where
    P: Puzzle,
{
    pub fn new(hidden_layers: Vec<usize>) -> Self {
        let mut layers = Vec::new();
        let mut length = P::FEATURE_LENGTH;

        for l in hidden_layers {
            layers.push(Layer::new(length, l));
            length = l;
        }

        layers.push(Layer::new(length, P::ACTIONS_LENGTH));

        Self {
            layers,
            _puzzle: PhantomData,
        }
    }

    pub fn randomize(&mut self, rng: &mut ThreadRng) {
        for layer in &mut self.layers {
            layer.randomize(rng);
        }
    }

    pub fn apply(&self, state: P) -> Vec<f64> {
        let mut features = state.get_features();
        for layer in &self.layers {
            features = layer.apply(features);
        }
        features
    }
}
