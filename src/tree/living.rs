use crate::tree::cell::Cell;
use rand::rngs::ThreadRng;

pub trait Living {
    fn evolve(self, rng: &mut ThreadRng, time: f32) -> Cell;
}
