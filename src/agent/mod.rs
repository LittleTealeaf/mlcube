mod epoch_function;
mod replay;

pub use replay::*;

use serde::{Deserialize, Serialize};

use crate::{network::Network, puzzle::Puzzle};

#[derive(Serialize, Deserialize)]
pub struct Agent<P>
where
    P: Puzzle,
{
    network: Network<P>,
    target: Network<P>,
    iter: usize,
    gamma: f64,
    update_interval: usize,
    replay_strategy: ReplayStrategy,
}
