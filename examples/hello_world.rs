//! Connects two nodes and returns a PNG image.

extern crate pathfinder;

use pathfinder::*;
use std::path::Path;

fn main() {
    let ls = 35; // Letter spacing.
    let hs = ls/2; // Letter spacing half.

    // TODO improve, by using a SVG or something.
    // Each line represents one character, spelling out:
    // P A T H F I N D E R
    let node_pos: Vec<(i16, i16)> = vec!(
        (0,0), (hs, -hs), (0, -ls), (0, ls),
        (ls, -hs), (ls+hs, ls), (hs, 0), (ls*2 +hs, 0),
        (ls*2, -hs), (ls*2, ls), (ls*2 +hs, ls),
        (ls*3, -hs), (ls*3, ls), (ls*3, hs), (ls*3 +hs, hs), (ls*3 +hs, ls),
        (ls*4, ls), (ls*4, -hs), (ls*4 +hs, -hs), (ls*4, -hs), (ls*4, 0), (ls*4 +hs, 0),
        (ls*5 -hs/2, hs/2), (ls*5 -hs/4, hs/2), (ls*5, hs/2), (ls*5, ls), (ls*5 +hs/2, ls),
        (ls*6 -hs, 0), (ls*6, ls), (ls*6 +hs/3, 0),
        (ls*7 -hs, -hs), (ls*7 -hs, ls), (ls*7, ls -hs), (ls*7 -hs, hs/2),
        (ls*8, 0), (ls*8 -hs, -hs), (ls*8 -ls, 0), (ls*8 -hs, ls), (ls*8 +hs, ls),
        (ls*9 -hs, -hs/3), (ls*9 -hs, 0), (ls*9, 0), (ls*9, hs/3)
    );
    let nodes = Node::from_list(&node_pos);
    let mut nodes = Node::linked_list(nodes);
    for node in nodes.iter_mut() {
        node.color = tools::gen_rgba();
    }

    let path = Path::new("out.png");
    let mut map = Map::new();
    map = map.map(&nodes);
    let _ = map.image.unwrap().save(&path);
}
