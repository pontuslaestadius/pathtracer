//! Connects two nodes and returns a PNG image.

extern crate pathfinder;

use pathfinder::*;
use std::path::Path;

fn main() -> Result<(), std::io::Error> {
    let pos = vec![(0, -100), (0, 0), (300, 0)];
    let wrapper = Node::linked_list(Node::from_list(&pos));

    let mut pos = Vec::new();
    let y = vec![5, 30, 45, 35, 40, 80, 75, 70, 25, 30];
    let spread = 300 / (y.len() - 1) as i16;
    for (i, y) in y.iter().enumerate() {
        pos.push((i as i16 * spread, -*y));
    }
    let line = Node::linked_list(Node::from_list(&pos));

    Map::new()
        .map(&wrapper)
        .map(&line)
        .save(&Path::new("out.png"))
}
