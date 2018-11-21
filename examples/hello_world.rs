extern crate pathfinder;

use pathfinder::*;
use std::{env, path::Path};

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        panic!("Can't plant nodes without config file.");
    }
    let file = &args[1];
    let path = Path::new(file);
    let nodes = Node::from_file(path.to_str().unwrap());
    let mut nodes = Node::linked_list(nodes?);
    for (i, node) in nodes.iter_mut().enumerate() {
        node.color = tools::seed_rgba(32 * i as u64);
    }
    Map::new().map(&nodes).save(&Path::new("out.png"))
}
