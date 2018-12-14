extern crate pathfinder;

use pathfinder::*;
use std::path::Path;

fn main() -> Result<(), std::io::Error> {
    let mut nodes = Node::linked_list(Node::from_list(&[(0, 0), (50, 50), (100, 0), (150, 50)]));
    nodes[0].hl_mut(0).unwrap().style(0);
    nodes[1].hl_mut(0).unwrap().style(1);
    nodes[2].hl_mut(0).unwrap().style(2);
    Map::new().map(&nodes).save(&Path::new("out.png"))
}
