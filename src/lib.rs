mod coordinates;

mod pathfinder;

extern crate rand;

use pathfinder::node::*;
use rand::distributions::{IndependentSample, Range};
use std::fs::File;
use std::io;
use std::io::prelude::*;

use coordinates::Coordinates;

use pathfinder::map;

pub fn create_network(number: u32, radius: i16) {

    // Stores all created nodes. So then they can be made in to a network.
    let mut nodes: Vec<Node> = Vec::new();

    // A list of all the names the nodes will be generated from.
    let node_names: Vec<String> = match get_node_names() {
        Ok(t) => t,
        Err(e) => {
            println!("Error: {}", e);
            panic!("Error is unrecoverable");
        },
    };

    // Generates a random Coordinates location.
    let mut c: Coordinates = Coordinates::new(0,0);

    // For the number of nodes in the network.
    for _ in 0..number {

        // Gets a name for the node.
        let name: String = get_random_item(&node_names).clone();

        nodes.push(Node::new(name,c.clone()));

        // Generates a location within a range of the previous one.
        c = Coordinates::gen_within_radius(c.clone(), radius);
    }

    let connections: Vec<NodeLink> = NodeLink::link(&nodes);

    for con in connections.iter() {
        con.save();
    }

    map::node_map(&nodes);

    for node in nodes.iter() {
        node.save();
    }
}


pub fn load() {
    pathfinder::node::Node::load();
}

// Opens
pub fn get_node_names() -> Result<Vec<String>, io::Error> {
    let mut file = File::open(constants::NAMEPATH)?;

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



/*
   Constants
*/

pub mod constants {
    pub static NODEPATH: &str = "resources/nodes.txt";
    pub static LINKPATH: &str = "resources/links.txt";
    pub static NAMEPATH: &str = "resources/nodenames.txt";
    pub static DEBUGMODE: bool = true;
}


#[cfg(test)]
mod tests {

    use pathfinder::*;

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

        let co4: Coordinates = Coordinates::new(102, 102);

        // Since randomness is applied. It's effect is lowered by using many iterations.
        for _ in 0..100 {
            let co5: Coordinates = Coordinates::gen_within_radius(co1.clone(), 100);
            let res4: bool = co4 > co5;
            assert_eq!(res4, true)
        }

    }

}



