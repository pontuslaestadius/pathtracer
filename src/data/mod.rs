use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

use group::*;
use node::coordinates::*;

use std::fs::OpenOptions;
use std::io::prelude::*;
use super::tools::util;
use super::node::link::*;


pub struct Tag {
    // Tag to divide them in to groups.
    pub collection: String,
    pub ignore: Vec<String>,
}

pub fn convert_file<'a>(path: &str, tag: &Tag) -> (Vec<Group>, Vec<Link<'a>>) {
    let mut file = OpenOptions::new()
        .read(true)
        .open(path)
        .unwrap();

    let mut contents = String::new();
    let _ = file.read_to_string(&mut contents);

    convert(contents, tag)
}

pub fn convert<'a>(content: String, tag: &Tag) -> (Vec<Group>, Vec<Link<'a>>) {
    let mut groups: Vec<Group> = Vec::new();
    let links: Vec<Link> = Vec::new();
    let lines = content.split("\n");

    // Stores the hashed array position rem.
    let size: usize = 10000;
    let size_u64: u64 = size as u64;
    let mut groups_boolean_array: [bool; 10000] = [false; 10000]; // TODO improve.

    let mut i = 0;
    for line in lines {
        // Ignore empty lines.
        if line == "" {continue};
        let radius = (size/20) as u32;

        // Pick up tagged lines.
        if line.starts_with(tag.collection.as_str()) {
            i+=1;

            // Check if a group matches the same.
            let coordinates = Coordinate::new(0, 0);

            // Hashes the input value for faster comparison.
            let hashed_line = calculate_hash(&line);

            // Checks the boolean array position for the groups existence.
            if groups_boolean_array[(hashed_line % size_u64) as usize] {

                for old in &mut groups.iter_mut() {
                    // If it does not match existing tag.
                    if old.hash != hashed_line {continue};
                    let _ = old.new_node_min_auto(String::new(), 100);
                    // Draw a line between the previous and the next commit if they are chained.
                    break;
                }

                // Creates a new group because one did not exist.
            } else {
                // Sets the group to exists in the boolean array.
                groups_boolean_array[(hashed_line % size_u64) as usize] = true;
                // Produce a new group.
                let mut group = Group::new(
                    &line,
                    gen_radius(&coordinates, 0, radius*2),
                    util::gen_rgba_reliable(calculate_hash(&line)),
                    None
                );

                let len = group.nodes.len() as u32;
                group.new_node_min_auto(String::new(), len);
                groups.push(group);
            }

            /*
            // Chain linking
            let group: &Group = &groups.get(index).unwrap();
            if group.name == previous_tag {
                let previous_node: &Node = group.nodes.get(group.nodes.len()-2).unwrap();
                links.push(Link::new(&previous_node.geo, &ref_node_geo.unwrap()));
                previous_tag = group.name.clone();
            }
            */
        }

    }

    (groups, links)
}

pub fn calculate_hash<T: Hash>(t: &T) -> u64 {
    let mut s = DefaultHasher::new();
    t.hash(&mut s);
    s.finish()
}