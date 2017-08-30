pub mod node;
pub mod map;
pub mod tools;
pub mod network;

use std::fs::File;
use std::io;
use std::io::prelude::*;

use pathfinder::node::Node;

fn shortest_path(list: &[Node], from: &Node, to: &Node) {
    // TODO Implement when everything else is done.
}