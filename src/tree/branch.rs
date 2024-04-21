use std::collections::LinkedList;

use rand::rngs::ThreadRng;
use svg::{
    node::element::Line,
    Node,
};

use crate::{
    color_gen::branch_hex,
    consts::UP_ROT,
    paint_list::{GetPaintlist, PaintList},
};

use super::{cell::Cell, leaf::Leaf, living::Living, twig::Twig};

#[derive(Debug)]
pub struct Branch {
    pub age: f32,
    pub children: LinkedList<Box<Cell>>,
    pub angle: f32,
}

impl From<&mut Twig> for Branch {
    fn from(value: &mut Twig) -> Self {
        let range = 0..2;
        let mut children = LinkedList::new();
        for _ in range {
            children.push_front(Box::new(Cell::Leaf(Leaf::new())));
        }
        Branch {
            age: value.age - value.mature_at,
            children,
            angle: value.angle,
        }
    }
}

impl Living for Branch {
    fn evolve(mut self, rng: &mut ThreadRng, time: f32) -> Cell {
        self.age += time;
        let mut new_children = LinkedList::new();
        for old_child in self.children {
            new_children.push_back(Box::new(old_child.evolve(rng, time)));
        }
        self.children = new_children;
        Cell::Branch(self)
    }
}

impl GetPaintlist for Branch {
    fn get_paintlist(
        &self,
        base_pos: (f32, f32),
        base_angle: f32,
        rng: &mut ThreadRng
    ) -> PaintList {
        let l = self.age.sqrt();
        let angle = base_angle + self.angle + (UP_ROT - base_angle - self.angle).powi(3) * 0.02;
        let end_pos = (base_pos.0 + angle.cos() * l, base_pos.1 + angle.sin() * l);

        let b: Box<dyn Node> = Box::new(
            Line::new()
                .set("stroke", branch_hex(rng))
                .set("stroke-linecap", "round")
                .set("stroke-width", self.age / 200.0)
                .set("x1", base_pos.0)
                .set("y1", base_pos.1)
                .set("x2", end_pos.0)
                .set("y2", end_pos.1)
        );

        let mut paint_list = PaintList {
            branches: Some(LinkedList::from([b])),
            flowers: None,
        };

        for child in &self.children {
            let additional = child.get_paintlist(end_pos, angle, rng);
            match &mut paint_list.flowers {
                Some(ref mut l) => {
                    if let Some(flowers) = additional.flowers {
                        l.extend(flowers);
                    }
                }
                None => paint_list.flowers = additional.flowers,
            }
            match &mut paint_list.branches {
                Some(ref mut l) => {
                    if let Some(branches) = additional.branches {
                        l.extend(branches);
                    }
                }
                None => paint_list.branches = additional.branches,
            }
        }

        paint_list
    }
}
