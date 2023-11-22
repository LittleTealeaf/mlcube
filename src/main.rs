#![allow(dead_code)]
use agent::{AgentFactory, EpochFunction, ReplayStrategy};
use puzzle::eight::EightPuzzle;

mod agent;
mod network;
mod puzzle;
mod utils;

type _Puzzle = EightPuzzle;

const SCRAMBLE_DEPTH: usize = 500;
const REPLAY_SIZE: usize = 5_000;
const TRAIN_SIZE: usize = (REPLAY_SIZE * 2) / 3;

const UPDATE_INTERVAL: usize = 50;
const GAMMA: f64 = 0.9;

fn epsilon(_iter: usize) -> f64 {
    0.5
    // 0.7 * 0.9f64.powi((iter / UPDATE_INTERVAL) as i32)
}

fn alpha(iter: usize) -> f64 {
    0.95f64.powi((iter % UPDATE_INTERVAL + 1) as i32)
}

fn main() {
    let _agent = AgentFactory {
        hidden_layers: vec![150; 10],
        gamma: 0.9,
        alpha: EpochFunction::WithinTargetPow { scale: 0.95 },
        epsilon: EpochFunction::Const(0.5),
        replay_strategy: ReplayStrategy::EvenSample {
            scramble_depth: 200,
            instances: 20,
        },
        train_size: 1_000,
        update_interval: 50,
    }
    .build::<_Puzzle>()
    .unwrap();

    println!("{}", ron::to_string(&_agent).unwrap())
}
