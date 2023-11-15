mod layer;
use std::marker::PhantomData;

pub use layer::*;
use rand::{distributions::uniform::SampleRange, rngs::ThreadRng};

use crate::puzzle::Puzzle;

#[derive(Clone, Debug)]
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

    pub fn copy_size(&self) -> Self {
        let mut iter = self.layers.iter();
        iter.next();
        let hidden_layers = iter.map(|layer| layer.get_inputs()).collect();
        Self::new(hidden_layers)
    }

    pub fn randomize<R>(&mut self, rng: &mut ThreadRng, range: R)
    where
        R: SampleRange<f64> + Clone,
    {
        for layer in &mut self.layers {
            layer.randomize(rng, range.clone());
        }
    }

    pub fn apply(&self, state: P) -> Vec<f64> {
        let mut features = state.get_features();
        for layer in &self.layers {
            features = layer.apply(features);
        }
        features
    }

    pub fn back_propagate(&self, state: P, action: usize, expected: f64, alpha: f64) -> Vec<Layer> {
        struct Entry<'a> {
            features: Vec<f64>,
            outputs: Vec<f64>,
            layer: &'a Layer,
        }

        let mut entries = Vec::with_capacity(self.layers.len());
        let mut features = state.get_features();

        for layer in self.layers.iter() {
            let outputs = layer.apply(features.clone());
            entries.push(Entry {
                features,
                outputs: outputs.clone(),
                layer,
            });
            features = outputs;
        }

        let mut nudges = Vec::new();

        let mut errors = {
            let entry = entries.pop().unwrap();
            let error = expected - entry.outputs[action];

            let LayerBackPropagate { error, nudge } =
                entry
                    .layer
                    .back_propagate_output(entry.features, error, action);

            nudges.push(nudge);
            error
        };

        while let Some(Entry {
            features,
            outputs,
            layer,
        }) = entries.pop()
        {
            let LayerBackPropagate { error, nudge } =
                layer.back_propagate(features, &errors, &outputs);
            errors = error;
            nudges.push(nudge);
        }

        for nudge in &mut nudges {
            nudge.scale(alpha);
        }

        nudges.into_iter().rev().collect()
    }

    pub fn update_weights(&mut self, nudges: Vec<Layer>) {
        assert_eq!(self.layers.len(), nudges.len());
        for (index, layer) in nudges.into_iter().enumerate() {
            self.layers[index].add(layer);
        }
    }
}
