#![allow(dead_code)]
use std::fs;

use mlcube::{
    agent::{Agent, FnValue, NewAgentConfig, ReplayStrategy},
    network::SolveResult,
    puzzle::{environments::EightPuzzle, Puzzle},
};
use rand::{seq::SliceRandom, thread_rng};

const EVALUATE_INTERVAL: usize = 100;

fn main() {
    let mut agent: Agent<EightPuzzle> = Agent::new(NewAgentConfig {
        hidden_layers: vec![81; 10],
        gamma: 0.9,
        alpha: FnValue::from(0.95).exp((FnValue::Epoch % FnValue::UpdateInterval) + 1.0.into()),
        epsilon: FnValue::Const(0.9).exp(FnValue::Epoch / FnValue::UpdateInterval),
        replay_strategy: ReplayStrategy::RandomScrambleState {
            scramble_min: 1,
            scramble_max: 100,
            instances: 20,
            instance_replay_length: 100,
        },
        train_size: 500,
        update_interval: 100,
        initialize_range: -0.00001..0.00001,
    })
    .unwrap();

    loop {
        if agent.get_epoch() % 100 == 0 {
            fs::write("./agent.ron", ron::to_string(&agent).unwrap()).unwrap();
        }

        agent.train_epoch();

        if agent.has_inf_or_nan() {
            panic!("Ran into inf / NaN");
        }

        if agent.get_epoch() % EVALUATE_INTERVAL == 0 {
            println!("Epoch {}", agent.get_epoch());
            let mut puzzle = EightPuzzle::new();
            let mut rng = thread_rng();
            for _ in 0..10 {
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
