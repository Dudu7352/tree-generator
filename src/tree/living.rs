use crate::{tree::cell::Cell, tree_config::TreeConfig};
use rand::rngs::ThreadRng;

pub trait Living {
    fn evolve(self, rng: &mut ThreadRng, time: f32, tree_config: &TreeConfig) -> Cell;
}
