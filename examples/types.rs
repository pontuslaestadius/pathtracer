extern crate pathtracer;

use pathtracer::*;
use std::path::Path;

fn main() -> Result<(), std::io::Error> {
    let mut nodes = Node::linked_list(Node::from_list(&[(0, 0), (50, 50), (100, 0), (150, 50)]));
    nodes[0].hl_mut(0).unwrap().style(EdgeStyle::Direct);
    nodes[1].hl_mut(0).unwrap().style(EdgeStyle::Straight);
    nodes[2].hl_mut(0).unwrap().style(EdgeStyle::Ellipse);
    Map::new().map(&nodes).save(Path::new("out.png")).unwrap();
    Ok(())
}
