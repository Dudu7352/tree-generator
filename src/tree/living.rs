use rand::rngs::ThreadRng;
use crate::tree::cell::Cell;

pub trait Living {
    fn evolve(self, rng: &mut ThreadRng, time: f32) -> Cell;
}