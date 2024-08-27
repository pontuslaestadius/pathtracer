extern crate image;
extern crate pathtracer;

use pathtracer::*;
use std::path::Path;

fn main() -> Result<(), std::io::Error> {
    let mut group = cluster!();
    group.radius(200);
    group.add(50);
    group.radius(400);
    group.nodes = Node::linked_list(group.nodes);
    group.each(&|node: &mut Node| {
        if let Ok(e) = node.hl_mut(0) {
            e.style(EdgeStyle::Ellipse)
        }
    });
    Map::new().map(&[group]).save(Path::new("out.png")).unwrap();
    Ok(())
}
