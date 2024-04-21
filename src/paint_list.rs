use std::collections::LinkedList;

use rand::rngs::ThreadRng;
use svg::Node;

pub struct PaintList {
    pub flowers: Option<LinkedList<Box<dyn Node>>>,
    pub branches: Option<LinkedList<Box<dyn Node>>>,
}

pub trait GetPaintlist {
    fn get_paintlist(&self, base_pos: (f32, f32), base_angle: f32, rng: &mut ThreadRng) -> PaintList;
}