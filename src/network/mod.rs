mod activation;
mod layer;
use std::marker::PhantomData;

pub use layer::*;
use rand::{distributions::uniform::SampleRange, rngs::ThreadRng};
use serde::{Deserialize, Serialize};

use crate::{puzzle::Puzzle, utils::ArgMax};

#[derive(Clone, Debug, Serialize, Deserialize)]
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

        for layer in &self.layers {
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

            let layer_bp = entry
                .layer
                .back_propagate_output(entry.features, error, action);

            nudges.push(layer_bp.nudge);
            layer_bp.error
        };

        while let Some(Entry {
            features,
            outputs,
            layer,
        }) = entries.pop()
        {
            let layer_bp = layer.back_propagate(features, &errors, &outputs);
            errors = layer_bp.error;
            nudges.push(layer_bp.nudge);
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

    pub fn has_inf_or_nan(&self) -> bool {
        self.layers.iter().any(Layer::has_inf_or_nan)
    }
}

/// Once training is done
impl<P> Network<P>
where
    P: Puzzle,
{
    /// Attempts to solve the puzzle. Returns the moves taken to solve. If it loops and does not
    /// solve, returns [`None`]
    ///
    /// Also returns [`None`] if it takes more than 10000 moves.
    pub fn solve(&self, mut puzzle: P, max_moves: usize) -> SolveResult {
        let mut actions = Vec::new();
        let mut states = vec![puzzle.clone()];

        for _ in 0..max_moves {
            if puzzle.is_solved() {
                return SolveResult::Solved(actions);
            }
            let values = self.apply(puzzle.clone());
            let action = values.arg_max();
            actions.push(action);
            puzzle.apply(action).unwrap();

            if states.contains(&puzzle) {
                return SolveResult::Loop(actions);
            }

            states.push(puzzle.clone());
        }

        SolveResult::TimedOut
    }
}

#[derive(Debug)]
pub enum SolveResult {
    Solved(Vec<usize>),
    TimedOut,
    Loop(Vec<usize>),
}
