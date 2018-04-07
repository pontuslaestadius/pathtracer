use std::cmp::PartialEq;
use std::fs::{File, OpenOptions};
use std::io;
use std::io::prelude::*;
use super::{Node, Shape};

pub mod coordinates;
pub mod link;
pub mod figure;

impl<T: Shape> PartialEq for Node<T> {
    fn eq(&self, other: &Node<T>) -> bool {
        self.hash == other.hash
    }
}

/// Returns a list of names specified in a resource file.
pub fn get_node_names(path: &str) -> Result<Vec<String>, io::Error> {
    let mut file = File::open(path)?;
    let mut contents = String::new();
    let mut names: Vec<String> = Vec::new();

    file.read_to_string(&mut contents)?;
    let split = contents.split('\n');
    for value in split {
        names.push(value.to_string());
    }
    Ok(names)
}

/*
/// Saves a list of nodes to a constant path.
/// It is more efficient to save several nodes at once.
pub fn save_list(list: &[Node]) -> Result<(), io::Error> {
    // Opens the node file.
    let mut file: File = OpenOptions::new()
        .create(true)
        .append(true)
        .truncate(false)
        .open(constant::NODEPATH)?;
    for item in list {
        let str = [
            item.name.as_str(),
            ",",
            item.geo.x.to_string().as_str(),
            ",",
            item.geo.y.to_string().as_str(),
            "\n"
        ].concat();
        file.write_all(str.as_bytes())?;
    }
    Ok(())
}

/// Parses a static str to a Node.
pub fn parse(str: &str) -> Node {

    let string: String = str.to_string();

    let mut split = string.split(",");

    let name = split.next().unwrap().to_string();
    let x = split.next().unwrap().parse::<i16>().unwrap();
    let y = split.next().unwrap().parse::<i16>().unwrap();

    Node::new(name, Coordinate::new(x,y))
}

/// Loads from a constant path and returns all saved nodes.
pub fn load() -> Vec<Node> {

    let mut nodes: Vec<Node> = Vec::new();

    let mut file = OpenOptions::new()
        .read(true)
        .open(constant::NODEPATH)
        .unwrap();

    let mut contents = String::new();
    let _ = file.read_to_string(&mut contents);
    let split = contents.split('\n');


    for row in split {
        // Ignores things like empty lines, are anything that may be invalid.
        if row.len() > 15 {
            nodes.push(Node::parse(row));
        }
    }

    nodes
}
*/