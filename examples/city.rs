extern crate pathfinder;

use pathfinder::*;
use std::path::Path;

fn main() -> Result<(), std::io::Error> {
    let mut pos = Vec::new();
    let city_size = 30;
    let spread = 15;

    for y in 0..city_size / 2 {
        for x in 0..city_size * 2 {
            let mut node = Node::new(
                &format!("{},{}", x, y),
                Coordinate::new(spread * x as i16, spread * y as i16),
            );

            node.color = tools::seed_rgba((city_size * x + spread * y) as u64);
            pos.push(node);
        }
    }

    pos = Node::linked_list_predicate(pos, &|a, b| {
        let d = (a - b).abs().sum();
        d < spread * 3
    });

    Map::new()
        .map_filter(&pos, &|node: &Node| node.hl(0).is_ok())
        .save(&Path::new("out.png"))
}
