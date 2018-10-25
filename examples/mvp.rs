/*
    Creates a small list of nodes which are sequentially connected.
    Verifies that the initial node is connected with the last node.
*/

extern crate pathfinder;

use pathfinder::{node, *};

fn main() {
    let pos = [(0, 0), (100, 100), (150, 50), (100, 0)];

    let nodes = Node::from_list(&pos);
    let nodes = Node::linked_list(nodes);
    let net = Network::new(nodes);
    let path = net.path("D", "A");
    node::path_print(&path);
}
