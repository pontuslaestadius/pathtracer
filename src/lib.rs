extern crate rand;

mod avocado;
use avocado::node::*;

use rand::distributions::{IndependentSample, Range};
use std::fs::File;
use std::io;
use std::io::prelude::*;

pub fn create_network(number: u32) {

    // Stores all created nodes. So then they can be made in to a network.
    let mut nodes: Vec<Node> = Vec::new();

    // A list of all the names the nodes will be generated from.
    let node_names: Vec<String> = match get_node_names() {
        Ok(t) => t,
        Err(_) => panic!("Couldn't load nodenames"),
    };

    // Generates a random Coordinates location.
    let mut c: Coordinates = Coordinates::gen();

    // For the number of nodes in the network.
    for _ in 0..number {

        // Gets a name for the node.
        let name: String = get_random_item(&node_names).clone();

        nodes.push(Node::new(name,c.clone()));

        // Generates a location within a range of the previous one.
        c = Coordinates::gen_within_radius(c.clone(), 1000);
    }

    let connections: Vec<NodeLink> = NodeLink::link(&nodes);

    for con in connections.iter() {
        con.save();
    }

    for node in nodes.iter() {
        node.save();
    }

}


pub fn load() {
    Node::load();
}

// Opens
pub fn get_node_names() -> Result<Vec<String>, io::Error> {
    let mut file = File::open(avocado::node::NAMEPATH)?;

    let mut contents = String::new();
    file.read_to_string(&mut contents);
    let split = contents.split('\n');

    let mut names: Vec<String> = Vec::new();

    for value in split {
        names.push(value.to_string());
    }
    Ok(names)
}


// Returns a random item from a given list.
pub fn get_random_item(list: &[String]) -> &String {
    let mut rng = rand::thread_rng();
    let between: Range<usize> = Range::new(0, list.len());
    let roll = between.ind_sample(&mut rng);
    &list[roll]
}


#[cfg(test)]
mod tests {

    use avocado::node::*;

    #[test]
    fn coordinates() {
        let co1: Coordinates = Coordinates::new(1, 1);
        let co2: Coordinates = co1.clone();
        let co3: Coordinates = Coordinates::new(2, 2);

        // Comparing Coordinates
        let res1: bool = co1 == co2;
        let res2: bool = co1 == co3;
        let res3: bool = co1 < co3;

        assert_eq!(res1, true);
        assert_ne!(res2, true);
        assert_eq!(res3, true);
    }

}



