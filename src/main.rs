#![allow(dead_code)]
use std::fs;

use mlcube::{
    agent::{AgentFactory, ReplayStrategy, Value},
    network::SolveResult,
    puzzle::{environments::EightPuzzle, Puzzle},
};
use rand::{seq::SliceRandom, thread_rng};

fn main() {
    let mut agent = AgentFactory {
        hidden_layers: vec![100; 20],
        gamma: 0.9,
        alpha: Value::Const(0.95).exp((Value::Epoch % Value::UpdateInterval) + Value::Const(1f64)),
        epsilon: Value::Const(0.5)
            .exp((Value::Epoch % Value::UpdateInterval) + Value::Const(1f64)),
        replay_strategy: ReplayStrategy::ScrambledState {
            scramble_depth: 100,
            instances: 50,
            instance_replay_length: 100,
        },
        train_size: 1000,
        update_interval: 50,
        initialize_range: -0.00001..0.00001,
    }
    .build::<EightPuzzle>()
    .unwrap();

    loop {
        if agent.get_epoch() % 100 == 0 {
            fs::write("./agent.ron", ron::to_string(&agent).unwrap()).unwrap();
        }

        agent.train_epoch();

        if agent.has_inf_or_nan() {
            panic!("Ran into inf / NaN");
        }

        if agent.get_epoch() % 20 == 0 {
            println!("Epoch {}", agent.get_epoch());
            let mut puzzle = EightPuzzle::new();
            let mut rng = thread_rng();
            for _ in 0..500 {
                puzzle
                    .apply(*puzzle.get_valid_actions().choose(&mut rng).unwrap())
                    .unwrap();
            }

            match agent.solve(puzzle.clone(), 10_000) {
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
}
