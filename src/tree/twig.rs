use std::collections::LinkedList;

use rand::{rngs::ThreadRng, Rng};
use svg::{
    node::element::{path::Data, Circle, Path},
    Node,
};

use crate::{
    color_gen::{branch_hex, flower_hex},
    consts::UP_ROT,
    paint_list::{GetPaintlist, PaintList},
};

use super::{branch::Branch, cell::Cell, living::Living};

#[derive(Debug, Default)]
pub struct Twig {
    pub age: f32,
    pub mature_at: f32,
    pub angle: f32,
}

impl Twig {
    pub fn new(rng: &mut ThreadRng, base_age: f32, custom_angle: Option<f32>) -> Self {
        Self {
            age: base_age,
            mature_at: rng.gen_range(15.0..25.0),
            angle: custom_angle.unwrap_or(rng.gen_range(-0.35..0.35) * 2.0),
        }
    }
}

impl Living for Twig {
    fn evolve(mut self, rng: &mut ThreadRng, time: f32) -> Cell {
        let old_age = self.age;
        self.age += time;
        if self.age > self.mature_at {
            return Branch::from(&mut self).evolve(rng, time + old_age - self.mature_at);
        }
        Cell::Twig(self)
    }
}

impl GetPaintlist for Twig {
    fn get_paintlist(
        &self,
        base_pos: (f32, f32),
        base_angle: f32,
        rng: &mut ThreadRng,
    ) -> PaintList {
        let l = self.age.sqrt();
        let angle = base_angle + self.angle + (UP_ROT - base_angle - self.angle).powi(3) * 0.02;
        let end_pos = (base_pos.0 + l * angle.cos(), base_pos.1 + l * angle.sin());
        let data = Data::new().move_to(base_pos).line_to(end_pos).close();
        let circle_b: Box<dyn Node> = Box::new(
            Circle::new()
                .set("r", 0.7)
                .set("cx", end_pos.0)
                .set("cy", end_pos.1)
                .set("fill", flower_hex(rng)),
        );
        let b: Box<dyn Node> = Box::new(
            Path::new()
                .set("fill", "none")
                .set("stroke", branch_hex(rng))
                .set("stroke-width", self.age / 200.0)
                .set("stroke-linecap", "round")
                .set("d", data),
        );
        PaintList {
            flowers: Some(LinkedList::from([circle_b])),
            branches: Some(LinkedList::from([b])),
        }
    }
}
