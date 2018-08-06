use std::cmp::PartialEq;
use std::fs::File;
use std::io;
use std::io::prelude::*;
use super::{Node, Shape, Coordinate};

pub mod coordinates;

impl<'a> PartialEq for Node<'a> {
    fn eq(&self, other: &Node) -> bool {
        self.hash == other.hash
    }
}

/// Returns a list of Strings split using \n in a Vec.
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

/// Parses a static str to a Node.
///
/// ```
/// use pathfinder::node;
/// use pathfinder::Square;
/// let node_str = "name,100,50";
/// let node = node::parse::<Square>(node_str);
/// assert_eq!(node.geo.x, 100);
/// assert_eq!(node.geo.y, 50);
/// ```
pub fn parse<T: Shape>(str: &str) -> Node {

    let string: String = str.to_string();

    let mut split = string.split(",");

    let name = split.next().unwrap().to_string();
    let x = split.next().unwrap().parse::<i16>().unwrap();
    let y = split.next().unwrap().parse::<i16>().unwrap();

    Node::new(&name, Coordinate::new(x,y))
}

