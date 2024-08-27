extern crate pathtracer;

use pathtracer::*;
use std::{env, path::Path};

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        panic!("Can't plant nodes without config file.");
    }
    let nodes = Node::from_file(&args[1])?;
    let mut nodes = Node::linked_list(nodes);

    for (i, node) in nodes.iter_mut().enumerate() {
        node.color = tools::seed_rgb(32 * i as u64);
    }
    Map::new().map(&nodes).save(Path::new("out.png")).unwrap();
    Ok(())
}
