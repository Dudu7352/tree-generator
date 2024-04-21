use core::fmt::Debug;
use rand::rngs::ThreadRng;

use crate::{
    paint_list::{GetPaintlist, PaintList},
    tree::leaf::Leaf,
};

use super::{branch::Branch, living::Living, twig::Twig};

pub enum Cell {
    Leaf(Leaf),
    Twig(Twig),
    Branch(Branch),
}

impl GetPaintlist for Cell {
    fn get_paintlist(
        &self,
        base_pos: (f32, f32),
        base_angle: f32,
        rng: &mut ThreadRng,
    ) -> PaintList {
        match self {
            Cell::Leaf(v) => v.get_paintlist(base_pos, base_angle, rng),
            Cell::Twig(v) => v.get_paintlist(base_pos, base_angle, rng),
            Cell::Branch(v) => v.get_paintlist(base_pos, base_angle, rng),
        }
    }
}

impl Default for Cell {
    fn default() -> Self {
        Cell::Leaf(Leaf::new())
    }
}

impl Debug for Cell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Leaf(arg0) => f.write_str(&format!("{:?}", arg0)),
            Self::Twig(arg0) => f.write_str(&format!("{:?}", arg0)),
            Self::Branch(arg0) => f.write_str(&format!("{:?}", arg0)),
        }
    }
}

impl Living for Cell {
    fn evolve(self, rng: &mut ThreadRng, time: f32) -> Cell {
        match self {
            Cell::Leaf(v) => v.evolve(rng, time),
            Cell::Twig(v) => v.evolve(rng, time),
            Cell::Branch(v) => v.evolve(rng, time),
        }
    }
}
