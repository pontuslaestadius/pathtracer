extern crate pathfinder;
use pathfinder::{node, *};

/*
    Creates a small list of nodes which are sequentially connected.
    Verifies that the initial node is connected with the last node.
*/

fn main() -> std::io::Result<()> {
    let pos = [(0, 0), (100, 100), (150, 50), (2000, 4000), (400, 600)];

    let nodes = Node::from_list(&pos);
    let nodes = Node::linked_list(nodes);
    let net = Network::new(nodes);
    let path = net.path("A", "E");
    let rev_path = net.path_rev("E", "A"); // It's in the game.

    node::path_print(&path?);
    node::path_print(&rev_path?);
    Ok(())
}
