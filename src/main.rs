#![allow(dead_code)]
use std::fs;

use mlcube::{
    agent::{Agent, FnValue, NewAgentConfig, SampleStrategy, UpdateStrategy},
    network::SolveResult,
    puzzle::{environments::*, Puzzle},
};
use rand::{seq::SliceRandom, thread_rng};

const EVALUATE_INTERVAL: usize = 100;

type _Puzzle = LightsOut<3, 3>;

fn main() {
    let mut agent: Agent<_Puzzle> = Agent::new(NewAgentConfig {
        hidden_layers: vec![40, 40, 40],
        gamma: 0.9,
        alpha: FnValue::from(0.5)
            * FnValue::from(0.995).exp((FnValue::Epoch - FnValue::LastTargetUpdate) + 1.0.into()),
        epsilon: FnValue::from(0.1)
            + (FnValue::from(0.75)
                * FnValue::from(0.95).exp(
                    ((FnValue::Epoch - FnValue::LastTargetUpdate) / 2.0.into())
                        + FnValue::TargetUpdateCount
                        + FnValue::from(1.0),
                )),
        sample_strategy: SampleStrategy::ForcedIterative {
            target_updates_per_step: 5,
            instances: 10,
            instance_replay_length: 10,
        },
        batch_size: 2048,
        initialize_range: -0.1..0.1,
        update_strategy: UpdateStrategy::TrainThreshold {
            test_size: 100,
            initial_update: Some(100),
            min_update: Some(500),
            max_update: Some(5_000),
            threshold: 0.001,
        },
        max_replay_size: 100_000_000,
        penalize_repeats: false,
        experimental_update: true,
    });

    loop {
        if agent.get_epoch() % 10_000 == 0 {
            fs::write("./agent.ron", ron::to_string(&agent).unwrap()).unwrap();
        }

        agent.train_epoch();

        if agent.has_inf_or_nan() {
            panic!("Ran into inf / NaN");
        }

        if agent.get_epoch() % EVALUATE_INTERVAL == 0 {
            println!();
            println!(
                "Epoch {} | Target Updates: {}",
                agent.get_epoch(),
                agent.get_target_update_count()
            );
            let mut puzzle = _Puzzle::new();
            let mut rng = thread_rng();
            let mut max_steps = 0;

            for steps in 1..100 {
                puzzle
                    .apply(*puzzle.get_valid_actions().choose(&mut rng).unwrap())
                    .unwrap();

                match agent.solve(puzzle, 100) {
                    SolveResult::Solved(moves) => {
                        println!("\t {} | {:?}", puzzle, moves);
                        max_steps = steps;
                    }
                    SolveResult::TimedOut => {
                        break;
                    }
                    SolveResult::Loop(moves) => {
                        println!("\t {} | Looped: {:?}", puzzle, moves);
                        break;
                    }
                }
            }

            println!("Solved up to {max_steps} moves away");
            println!("{:?}", agent.get_network().apply(puzzle));
            println!("Average Error: {}", agent.test_target_error(100));
        }
    }
}
