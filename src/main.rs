#![allow(dead_code)]
use std::fs;

use mlcube::{
    agent::{Agent, FnValue, NewAgentConfig, SampleStrategy, UpdateStrategy},
    network::SolveResult,
    puzzle::{environments::*, Puzzle},
};
use rand::{seq::SliceRandom, thread_rng};

const EVALUATE_INTERVAL: usize = 20;

type _Puzzle = LightsOut<2, 2>;

fn main() {
    let mut agent: Agent<_Puzzle> = Agent::new(NewAgentConfig {
        hidden_layers: vec![10; 3],
        gamma: 0.9,
        alpha: FnValue::from(0.1)
            * FnValue::from(0.99).exp((FnValue::Epoch - FnValue::LastTargetUpdate) + 1.0.into()),
        epsilon: FnValue::from(0.2)
            + (FnValue::Const(0.7)
                * FnValue::Const(0.95).exp(
                    FnValue::TargetUpdateCount
                        + (FnValue::Epoch - FnValue::LastTargetUpdate)
                        + 1.0.into(),
                )),
        sample_strategy: SampleStrategy::ForcedIterative {
            target_updates_per_step: 10,
            instances: 24,
            instance_replay_length: 50,
        },
        batch_size: 1024,
        initialize_range: -0.001..0.001,
        update_strategy: UpdateStrategy::TrainThreshold {
            test_size: 1_000,
            initial_update: Some(100),
            min_update: Some(100),
            max_update: Some(10_000),
            threshold: 0.001,
        },
        max_replay_size: 10_000,
        penalize_repeats: false,
    })
    .unwrap();

    loop {
        if agent.get_epoch() % 10_000 == 0 {
            fs::write("./agent.ron", ron::to_string(&agent).unwrap()).unwrap();
        }

        agent.train_epoch();

        if agent.has_inf_or_nan() {
            panic!("Ran into inf / NaN");
        }

        if agent.get_epoch() % EVALUATE_INTERVAL == 0 {
            println!(
                "Epoch {} | Target Updates: {}",
                agent.get_epoch(),
                agent.get_target_update_count()
            );
            let mut puzzle = _Puzzle::new();
            let mut rng = thread_rng();
            for _ in 0..100 {
                puzzle
                    .apply(*puzzle.get_valid_actions().choose(&mut rng).unwrap())
                    .unwrap();
            }

            println!("{:?}", agent.get_network().apply(puzzle));

            match agent.solve(puzzle, 100) {
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
            println!("Average Error: {}", agent.test_target_error(1_000));
        }
    }
}
