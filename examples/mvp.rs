/*
    Creates a small list of nodes which are sequentially connected.
    Verifies that the initial node is connected with the last node.
*/

extern crate pathfinder;

use pathfinder::{node, *};

fn main() {
    let pos = [(0, 0), (100, 100), (150, 50), (2000, 4000), (400, 600)];

    let nodes = Node::from_list(&pos);
    let nodes = Node::linked_list(nodes);
    let net = Network::new(nodes);
    let path = net.path("E", "A"); // It's in the game.

    node::path_print(&path);
}
