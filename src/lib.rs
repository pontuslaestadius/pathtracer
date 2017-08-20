extern crate rand;

mod avocado;
use avocado::node::*;

use rand::distributions::{IndependentSample, Range};

use std::cmp::PartialEq;

use std::fs::File;
use std::fs::OpenOptions;

use std::io::prelude::*;
use std::io;

pub fn create_network(number: u32) {

    // Stores all created nodes. So then they can be made in to a network.
    let mut nodes: Vec<Node> = Vec::new();

    // A list of all the names the nodes will be generated from.
    let node_names: Vec<String> = get_node_names();

    // Generates a random Coordinates location.
    let mut c: Coordinates = Coordinates::gen();

    // For the number of nodes in the network.
    for _ in 0..number {

        // Gets a name for the node.
        let name: String = get_random_item(&node_names).clone();


        nodes.push(Node::new(name,c.clone()));


        // Links the nodes together as a network.
        if nodes.len() > 1 {                // Needs more than 1 item to link.
            let len = nodes.len();

            nodes[len-2].link(&nodes[len-1]);
        }

        // Generates a location within a range of the previous one.
        c = Coordinates::gen_within_radius(c.clone(), 1000);
    }

    for node in nodes.iter() {
        node.save();
    }

}

pub fn load() {
    Node::load();
}

pub fn get_node_names() -> Vec<String> {
    let mut file = match File::open("nodenames") {
        Ok(t) => t,
        Err(_) => panic!("Couldn't load nodenames"),
    };

    let mut contents = String::new();
    file.read_to_string(&mut contents);
    let mut split = contents.split('\n');

    let mut names: Vec<String> = Vec::new();

    for value in split {
        names.push(value.to_string());
    }
    names
}


pub fn get_random_item(list: &[String]) -> &String {
    let mut rng = rand::thread_rng();

    // Randomly gets the radius of the circle.
    let between: Range<usize> = Range::new(0, list.len());
    let roll = between.ind_sample(&mut rng);
    &list[roll]
}
