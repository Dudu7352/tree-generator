use std::collections::LinkedList;

use rand::{rngs::ThreadRng, Rng};
use svg::{node::element::Circle, Node};

use crate::{color_gen::flower_hex, paint_list::{GetPaintlist, PaintList}};

use super::{cell::Cell, living::Living, twig::Twig};

#[derive(Debug)]
pub struct Leaf {
    pub age: f32,
    pub mature_at: f32,
}

impl Leaf {
    pub fn new() -> Self {
        let mut rng = rand::thread_rng();
        Self {
            age: 0.0,
            mature_at: rng.gen_range(6.0..20.0),
        }
    }
}

impl Living for Leaf {
    fn evolve(mut self, rng: &mut ThreadRng, time: f32) -> Cell {
        let old_age = self.age;
        self.age += time;
        if self.age > self.mature_at {
            return Twig::new(rng, 0.0).evolve(rng, time + old_age - self.mature_at);
        }
        Cell::Leaf(self)
    }
}

impl GetPaintlist for Leaf {
    fn get_paintlist(&self, base_pos: (f32, f32), base_angle: f32, rng: &mut ThreadRng) -> PaintList {
        let circle = Circle::new()
            .set("r", 1.3)
            .set("cx", base_pos.0)
            .set("cy", base_pos.1)
            .set("fill", flower_hex(rng));
        let b: Box<dyn Node> = Box::new(circle);
        PaintList {
            flowers: Some(LinkedList::from([b])),
            branches: None,
        }
    }
}