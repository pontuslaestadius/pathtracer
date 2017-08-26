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

use util::debug_print;

pub fn create_network(number: u32, radius: i16) -> Result<(), io::Error> {
    debug_print("creating node network..");

    // Stores all created nodes. So then they can be made in to a network.
    let mut nodes: Vec<Node> = Vec::new();

    // A list of all the names the nodes will be generated from.
    let node_names: Vec<String> = get_node_names()?;

    // Generates a random Coordinates location.
    let mut c: Coordinates = Coordinates::new(0,0);


    debug_print("   creating nodes..");
    // For the number of nodes in the network.
    for _ in 0..number {

        let mut temp_nodes: Vec<Node> = Vec::new();
        for node in &nodes {
            let d = Coordinates::gen_within_radius(node.geo.clone(), radius);
            let name: String = get_random_item(&node_names).clone();

            temp_nodes.push(Node::new(name,d.clone()));

            // Generates a location within a range of the previous one.
            c = Coordinates::gen_within_radius(node.geo.clone(), radius);
        }
        for node in temp_nodes {
            nodes.push(node);
        }

        // Gets a name for the node.
        let name: String = get_random_item(&node_names).clone();

        nodes.push(Node::new(name,c.clone()));

        // Generates a location within a range of the previous one.
        c = Coordinates::gen_within_radius(c.clone(), radius);
    }
    debug_print("   done");

    debug_print("   linking nodes..");
    let connections: Vec<NodeLink> = NodeLink::link(&nodes);
    debug_print("   done");


    debug_print("   saving links..");
    for con in connections.iter() {
        con.save();
    }
    debug_print("   done");

    debug_print("   generating map..");
    map::node_map(&nodes);
    debug_print("   done");


    //let _ = Node::save_all(&nodes); // TODO fix

    debug_print("done");
    Ok(())
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

pub mod util {
    use constants;

    pub fn debug_print(str: &str) {
        if constants::DEBUGMODE {
            println!("{}", str);
        }
    }
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



