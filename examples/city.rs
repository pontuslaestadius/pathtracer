extern crate image;
extern crate pathtracer;

use pathtracer::*;
use std::path::Path;

fn main() -> Result<(), image::ImageError> {
    let mut pos = Vec::new();
    let city_size = 30;
    let spread = 15;

    for y in 0..city_size / 2 {
        for x in 0..city_size * 2 {
            let mut node = node!(spread * x, spread * y);

            node.color = tools::seed_rgb((city_size * x + spread * y) as u64);
            pos.push(node);
        }
    }

    pos = Node::linked_list_predicate(pos, &|a, b| {
        let abs = (a - b).abs();
        let d = abs.x + abs.y;
        d < spread * 3
    });

    Map::new()
        .map_filter(&pos, &|node: &Node| node.hl(0).is_ok())
        .save(Path::new("out.png"))
}
