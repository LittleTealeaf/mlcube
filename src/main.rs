#![allow(dead_code)]
use agent::{AgentFactory, EpochFunction, ReplayStrategy};
use puzzle::environments::EightPuzzle;

mod agent;
mod network;
mod puzzle;
mod utils;

type _Puzzle = EightPuzzle;

fn main() {
    let _agent = AgentFactory {
        hidden_layers: vec![150; 10],
        gamma: 0.9,
        alpha: EpochFunction::WithinTargetPow { scale: 0.95 },
        epsilon: EpochFunction::Const(0.5),
        replay_strategy: ReplayStrategy::ScrambledState {
            scramble_depth: 100,
            instances: 50,
            instance_replay_length: 100,
        },
        train_size: 1_000,
        update_interval: 50,
    }
    .build::<_Puzzle>()
    .unwrap();

    println!("{}", ron::to_string(&_agent).unwrap())
}
