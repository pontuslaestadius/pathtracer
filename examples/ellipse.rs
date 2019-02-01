extern crate pathfinder;

use pathfinder::*;
use std::path::Path;

fn main() -> Result<(), std::io::Error> {
    let mut group = cluster!();
    group.radius(200);
    group.add(50);
    group.radius(400);
    group.nodes = Node::linked_list(group.nodes);
    group.each(&|node: &mut Node| {
        match node.hl_mut(0) {
            Ok(e) => e.style(2),
            Err(_) => (),
        }
    });
    Map::new().map(&[group]).save(&Path::new("out.png"))
}
