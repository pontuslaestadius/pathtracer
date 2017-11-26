//! Connects two nodes and returns a PNG image.

extern crate pathfinder;

use pathfinder::map::*;


fn main() {

    let (nodes, links) =


    node_and_links(&nodes, &connections);

}