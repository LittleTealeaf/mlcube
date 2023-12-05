#![allow(dead_code)]
use std::fs;

use mlcube::{
    agent::{Agent, FnValue, NewAgentConfig, SampleStrategy},
    network::SolveResult,
    puzzle::{environments::LightsOut, Puzzle},
};
use rand::{seq::SliceRandom, thread_rng};

const EVALUATE_INTERVAL: usize = 20;

type _Puzzle = LightsOut<3, 3>;

fn main() {
    let mut agent: Agent<_Puzzle> = Agent::new(NewAgentConfig {
        hidden_layers: vec![9,9,9],
        gamma: 0.95,
        alpha: FnValue::from(0.1)
            * FnValue::from(0.975).exp((FnValue::Epoch % FnValue::UpdateInterval) + 1.0.into()),
        epsilon: FnValue::from(0.2)
            + (FnValue::Const(0.7)
                * FnValue::Const(0.9).exp(
                    (FnValue::Epoch / FnValue::UpdateInterval).floor()
                        + (FnValue::Epoch % FnValue::UpdateInterval)
                        + 1.0.into(),
                )),
        sample_strategy: SampleStrategy::EvenSample {
            scramble_depth: 20,
            instances: 24,
        },
        batch_size: 128,
        update_interval: 100,
        initialize_range: -0.001..0.001,
        max_replay_size: 24 * 50 * 500,
        penalize_repeats: false,
    })
    .unwrap();

    loop {
        if agent.get_epoch() % 500 == 0 {
            fs::write("./agent.ron", ron::to_string(&agent).unwrap()).unwrap();
        }

        agent.train_epoch();

        if agent.has_inf_or_nan() {
            panic!("Ran into inf / NaN");
        }

        if agent.get_epoch() % EVALUATE_INTERVAL == 0 {
            println!("Epoch {}", agent.get_epoch());
            let mut puzzle = _Puzzle::new();
            let mut rng = thread_rng();
            for _ in 0..100 {
                puzzle
                    .apply(*puzzle.get_valid_actions().choose(&mut rng).unwrap())
                    .unwrap();
            }

            println!("{:?}", agent.get_network().apply(puzzle));

            match agent.solve(puzzle, 10_000) {
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
