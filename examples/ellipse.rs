extern crate pathfinder;

use pathfinder::*;
use std::path::Path;

fn main() -> Result<(), std::io::Error> {
    let mut nodes = Node::linked_list(Node::from_list(&[
        (0, 0),
        (-100, -100),
        (100, 100),
        (100, 0),
    ]));
    for node in nodes.iter_mut() {
        node.hl_mut(0).unwrap().style(2);
    }
    Map::new().map(&nodes).save(&Path::new("out.png"))
}
