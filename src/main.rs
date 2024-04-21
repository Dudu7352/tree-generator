use crate::tree::living::Living;
mod color_gen;
mod consts;
mod paint_list;
mod tree;

use consts::UP_ROT;
use paint_list::GetPaintlist;
use rand::{rngs::ThreadRng, thread_rng};
use svg::Document;
use tree::{cell::Cell, twig::Twig};

fn main() {
    let mut rng: ThreadRng = thread_rng();
    let mut cell = Cell::Twig(Twig {
        age: 14.0,
        mature_at: 24.0,
        angle: 0.0,
    });
    cell = cell.evolve(&mut rng, 300.0);
    let mut document = Document::new().set("viewBox", (0, 0, 400, 400));

    let paint_list = cell.get_paintlist((200.0, 300.0), UP_ROT, &mut rng);
    if let Some(branches) = paint_list.branches {
        for i in branches {
            document = document.add(i);
        }
    }
    if let Some(flowers) = paint_list.flowers {
        for i in flowers {
            document = document.add(i);
        }
    }
    svg::save("test.svg", &document).unwrap();
}
