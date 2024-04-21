use crate::{tree::living::Living, tree_config::TreeConfig};
mod color_gen;
mod consts;
mod paint_list;
mod params;
mod tree;
mod tree_config;

use clap::Parser;
use consts::UP_ROT;
use paint_list::GetPaintlist;
use params::Params;
use rand::{rngs::ThreadRng, thread_rng};
use svg::Document;
use tree::{cell::Cell, twig::Twig};

fn main() {
    let params = Params::parse();
    let tree_config = TreeConfig::from(&params);

    let mut rng: ThreadRng = thread_rng();
    let mut cell = Cell::Twig(Twig {
        age: 14.0,
        mature_at: 24.0,
        angle: 0.0,
    });

    for i in 1..=params.images {
        cell = cell.evolve(&mut rng, params.time / params.images as f32, &tree_config);

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

        svg::save(format!("{}_{}.svg", params.image_prefix, i), &document).unwrap();
    }
}
