use crate::*;

#[test]
fn city() {}

#[test]
fn git_log() {}

#[test]
fn hello_world_gif() {}

#[test]
fn mvp() -> std::io::Result<()> {
    let pos = [(0, 0), (100, 100), (150, 50), (2000, 4000), (400, 600)];

    let nodes = Node::from_list(&pos);
    let nodes = Node::linked_list(nodes);
    let net = Network::new(nodes);
    let path = net.path("A", "E");
    let rev_path = net.path_rev("E", "A");

    node::path_print(&path?);
    node::path_print(&rev_path?);
    Ok(())
}

#[test]
fn random() {}

#[test]
fn ellipse() {}

#[test]
fn graph() {}

#[test]
fn hello_world() {}

#[test]
fn node_plot() {}

#[test]
fn types() {}
