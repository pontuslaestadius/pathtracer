//! Connects two nodes and returns a PNG image.

extern crate pathfinder;

use pathfinder::*;
use std::path::Path;

fn main() -> Result<(), std::io::Error> {
    let pos = vec![(0, -100), (0, 0), (300, 0)];
    let wrapper = Node::linked_list(Node::from_list(&pos));

    let mut pos = Vec::new();
    let y = vec![10, 50, 40, 80, 90, 90, 50, 20, 30, 60];
    let spread = 300 / y.len() as i16;
    for (i, y) in y.iter().enumerate() {
        pos.push((i as i16 * spread, *y));
    }
    let line = Node::linked_list(Node::from_list(&pos));

    Map::new()
        .map(&wrapper)
        .map(&line)
        .save(&Path::new("out.png"))
}
