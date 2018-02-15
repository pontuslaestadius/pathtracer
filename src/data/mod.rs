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

pub fn convert_file<'a>(path: &str, lambda: &Fn(&str) -> bool) -> (Vec<Group>, Vec<Link<'a>>) {
    let mut file = OpenOptions::new()
        .read(true)
        .open(path)
        .unwrap();

    let mut contents = String::new();
    let _ = file.read_to_string(&mut contents);

    convert(contents, &lambda)
}

pub fn convert<'a>(content: String, lambda: &Fn(&str) -> bool) -> (Vec<Group>, Vec<Link<'a>>) {

    let cct = CustomConverter {
        split: '\n',
        node_range: 100,
        radius: 50,
        size: 1000,
        lambda_tag: &lambda,
    };

    convert_inner(content, cct)
}


pub struct CustomConverter<'a> {
    split: char,
    node_range: u32,
    radius: u32,
    size: u64,
    lambda_tag: &'a Fn(&str) -> bool,
    //lambda_group: &'a Fn(&str) -> bool,
}

// Heavily customizable.
pub fn convert_inner<'a>(content: String, cct: CustomConverter) -> (Vec<Group>, Vec<Link<'a>>) {
    let mut groups: Vec<Group> = Vec::new();

    let lines = content.split(cct.split);

    // Check if a group matches the same.
    // Stores the hashed array position rem.
    let coordinates = Coordinate::new(0, 0);
    let mut groups_boolean_array: [bool; 1000] = [false; 1000]; // TODO improve.

    for line in lines {
        // Ignore empty lines.
        if line == "" {continue};

        // Pick up tagged lines.
        if (cct.lambda_tag)(line) {

            // Hashes the input value for faster comparison.
            let hashed_line = calculate_hash(&line);

            // Checks the boolean array position for the groups existence.
            if groups_boolean_array[(hashed_line % cct.size) as usize] {

                for old in &mut groups.iter_mut() {
                    // If it does not match existing tag.
                    if old.hash != hashed_line {continue};
                    let _ = old.new_node_min_auto(String::new(), cct.node_range);
                    break;
                }

                // Creates a new group because one did not exist.
            } else {
                // Sets the group to exists in the boolean array.
                groups_boolean_array[(hashed_line % cct.size) as usize] = true;
                // Produce a new group.
                let mut group = Group::new(
                    &line,
                    gen_radius(&coordinates, 0, cct.radius*2),
                    util::gen_rgba_reliable(calculate_hash(&line)),
                    None
                );

                let len = group.nodes.len() as u32;
                group.new_node_min_auto(String::new(), len);
                groups.push(group);
            }

        }

    }

    (groups, Vec::new())
}

pub fn calculate_hash<T: Hash>(t: &T) -> u64 {
    let mut s = DefaultHasher::new();
    t.hash(&mut s);
    s.finish()
}