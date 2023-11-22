#![allow(dead_code)]
use agent::{AgentFactory, EpochFunction, ReplayStrategy};
use puzzle::environments::EightPuzzle;

mod agent;
mod network;
mod puzzle;
mod utils;

fn main() {
    let mut _agent = AgentFactory {
        hidden_layers: vec![100; 5],
        gamma: 0.9,
        alpha: EpochFunction::WithinTargetPow { scale: 0.9 },
        epsilon: EpochFunction::Const(0.5),
        replay_strategy: ReplayStrategy::ScrambledState {
            scramble_depth: 100,
            instances: 50,
            instance_replay_length: 100,
        },
        train_size: 100,
        update_interval: 50,
    }
    .build::<EightPuzzle>()
    .unwrap();

    for i in 0..100 {
        println!("Epoch {i}");
        _agent.train_epoch();
    }

}
