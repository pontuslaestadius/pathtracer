extern crate pathfinder;
extern crate rand;

use pathfinder::*;
use rand::{thread_rng, Rng};
use std::path::Path;

fn main() -> Result<(), std::io::Error> {
    let mut rng = thread_rng();

    let mut pos = Vec::new();
    let city_size = 40;
    let spread = 10;
    for x in 0..city_size {
        for y in 0..city_size {
            let mut node = Node::new(
                &format!("{},{}", x, y),
                Coordinate::new(spread * x as i16, spread * y as i16),
            );
            node.color = tools::seed_rgba((city_size * x + spread * y) as u64);
            pos.push(node);
        }
    }

    for _ in 0..100 {
        rng.shuffle(&mut pos);
        pos = Node::linked_list_predicate(pos, &|a, b| {
            let d = a.diff(&b);
            let d = d.0 + d.1;
            d < spread * 3
        });
    }

    Map::new()
        .map_filter(&pos, &|node: &Node| node.get_link_avail_index() != 0)
        .save(&Path::new("out.png"))
}
