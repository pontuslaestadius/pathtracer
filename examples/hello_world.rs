//! Connects two nodes and returns a PNG image.

extern crate pathfinder;

use pathfinder::map::*;
use pathfinder::node::*;
use pathfinder::group::*;


fn main() {
    let group1 = Node::new("Exampl1", coordinates::Coordinate::new(0,0));
    let group2 = Node::new("Exampl2", coordinates::Coordinate::new(100,100));
    let vec = vec!(group1, group2);



    // Use the log directory and the tag to create the groups.
    let (groups, links) = data::convert_file(log, &lambda);
    // Map them to an RGBA Image and saves it.
    groups_and_links(&groups, &links, "example.png");
}