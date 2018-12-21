extern crate pathfinder;

use pathfinder::*;
use std::path::Path;

fn main() -> Result<(), std::io::Error> {
    let mut group = Group::new("", Coordinate::new(0, 0));
    group.radius(200);
    group.add(50);
    group.nodes = Node::linked_list(group.nodes);
    group.each(&|node: &mut Node| node.hl_mut(0).unwrap().style(2));
    Map::new().map(&[group]).save(&Path::new("out.png"))
}
