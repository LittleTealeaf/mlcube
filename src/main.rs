#![allow(dead_code)]
use std::fs;

use agent::{AgentFactory, EpochFunction, ReplayStrategy};
use puzzle::environments::EightPuzzle;
use rand::{seq::SliceRandom, thread_rng};

use crate::{network::SolveResult, puzzle::Puzzle};

mod agent;
mod network;
mod puzzle;
mod utils;

fn main() {
    let mut agent = AgentFactory {
        hidden_layers: vec![100; 3],
        gamma: 0.9,
        alpha: EpochFunction::WithinTargetPow {
            scale: 0.9,
            base: 0.5,
        },
        epsilon: EpochFunction::Const(0.5),
        replay_strategy: ReplayStrategy::EvenSample {
            scramble_depth: 500,
            instances: 10,
        },
        // replay_strategy: ReplayStrategy::ScrambledState {
        //     scramble_depth: 100,
        //     instances: 50,
        //     instance_replay_length: 100,
        // },
        train_size: 100,
        update_interval: 50,
    }
    .build::<EightPuzzle>()
    .unwrap();

    loop {
        if agent.get_epoch() % 20 == 0 {
            fs::write("./test.ron", ron::to_string(&agent).unwrap()).unwrap();
        }

        agent.train_epoch();

        if agent.has_inf_or_nan() {
            panic!("Ran into inf / NaN");
        }

        if agent.get_epoch() % 100 == 0 {
            println!("Epoch {}", agent.get_epoch());
            let mut puzzle = EightPuzzle::new();
            let mut rng = thread_rng();
            for _ in 0..500 {
                puzzle
                    .apply(*puzzle.get_valid_actions().choose(&mut rng).unwrap())
                    .unwrap();
            }

            match agent.solve(puzzle.clone()) {
                SolveResult::Solved(actions) => {
                    println!("Solved:");
                    println!("\t{}", puzzle);
                    for action in actions {
                        puzzle.apply(action).unwrap();
                        println!("\t{} | {}", puzzle, action);
                    }
                }
                SolveResult::TimedOut => println!("Timed Out"),
                SolveResult::Loop(actions) => {
                    println!("Loop:");
                    println!("\t{}", puzzle);
                    for action in actions {
                        puzzle.apply(action).unwrap();
                        println!("\t{} | {}", puzzle, action);
                    }
                }
            }
        }
    }

    // for i in 0..100 {
    //     println!("Epoch {i}");
    //     _agent.train_epoch();
    // }
}
