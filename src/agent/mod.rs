mod function;
mod replay;
mod epoch_function;

pub use function::*;
pub use replay::*;

use serde::{Deserialize, Serialize};

use crate::{network::Network, puzzle::Puzzle};

use self::function::Function;

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
    epsilon: Function,
    alpha: Function,
    replay_strategy: ReplayStrategy
}
