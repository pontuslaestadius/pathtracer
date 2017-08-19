extern crate rand;

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

    for _ in 0..number {

        let mut c: Coordinates = Coordinates::gen();
        let mut not_success: bool = true;

        while not_success {
            not_success = false;

            for pos in nodes.iter() {

                // Matches to see if there exist any nodes with the same coordinates.
                if c.eq(&pos.geo) {
                    break;
                    not_success = true;
                    c = Coordinates::gen();
                }

            }



        }

        let name: String = get_random_line_from_vec(&node_names).clone();

        nodes.push(Node::new(name,c));
    }

    for node in nodes.iter() {
        node.save();
    }

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


pub fn get_random_line_from_vec(list: &Vec<String>) -> &String {
    let roll = 4; // Decided by a fair roll of die.
    &list[roll]
}

pub struct Node {
    name: String,
    connections: Vec<TravelLeg>,
    geo: Coordinates,
}

pub struct Coordinates {
    x: i16,
    y: i16,
}

pub struct TravelLeg {
    node: Node,
    time: u32,
    distance: u32,
}

impl Node {
    pub fn new(name: String, geo: Coordinates) -> Node {
        Node {
            name,
            connections: Vec::new(),
            geo,
        }
    }
}

impl PartialEq for Coordinates {
    fn eq(&self, other: &Coordinates) -> bool {
        (self.x == other.x) && (self.y == other.y)
    }
}

impl Coordinates {
    fn gen() -> Coordinates {
        let tuple = rand::random::<(i32, i32)>();

        Coordinates {
            x: rand::random::<i16>(),
            y: rand::random::<i16>(),
        }
    }
}

impl Node {
    pub fn save(&self) {
        let path = "nodes/nodes.txt";

        // Opens the node file.
        let mut file: File = match OpenOptions::new()
            .create(true)
            .append(true)
            .truncate(false)
            .open(path) {
            Result::Ok(t) => t,
                        _ => panic!("Couldn't open path"),
        };

        let mut connections: String = String::new();
        connections.push_str("/");

        for leg in &self.connections {
            connections.push_str(leg.node.gen_id().as_str());
        }


        let str = [
            self.gen_id().as_str(),
            ",",
            self.name.as_str(),
            ",",
            connections.as_str(),
            ",",
            self.geo.x.to_string().as_str(),
            ",",
            self.geo.y.to_string().as_str(),
            "\n"
        ].concat();

        print!("Saving: {}", str.as_str());

        file.write_all(str.as_bytes()).expect("Couldn't save node");
    }

    pub fn gen_id(&self) -> String {
        let mut id_x = self.geo.x.to_string();
        let mut id_y = self.geo.y.to_string();

        let mut len_x = id_x.len() -2;
        let mut len_y = id_y.len() -2;

        // TODO this looks horrible. Fix this.
        if len_x < 2 {
            len_x = id_x.len();
        }

        if len_y < 2 {
            len_y = id_y.len();
        }


        let format_x: String = id_x.split_off(len_x);
        let format_y: String = id_y.split_off(len_y);

        let mut clone = self.name.clone();

        clone.split_off(4);

        [
            clone,
            format_x,
            format_y
        ].concat()
    }
}