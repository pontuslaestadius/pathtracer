use group::*;
use node::coordinates::*;
use node::Node;

use std::fs::OpenOptions;
use std::io::prelude::*;
use super::tools::util;


pub struct Tag {
    // Tag to divide them in to groups.
    pub collection: String,
    pub ignore: Vec<String>,
}

pub fn convert_file(path: &str, tag: &Tag) -> Vec<Group> {
    let mut file = OpenOptions::new()
        .read(true)
        .open(path)
        .unwrap();

    let mut contents = String::new();
    let _ = file.read_to_string(&mut contents);

    convert(contents, tag)
}

pub fn convert(mut content: String, tag: &Tag) -> Vec<Group> {
    let mut groups: Vec<Group> = Vec::new();
    let lines = content.split("\n");

    let mut i = 0;
    for line in lines {
        // Ignore empty lines.
        if line == "" {continue};

        if line.starts_with(tag.collection.as_str()) {
            i+=1;

            // Check if a group matches the same.
            let coordinates = Coordinates::new(0,0);
            let radius = 80;

            let mut exists = false;
            for mut old in &mut groups {
                if old.name == line {
                    exists = true;
                    old.new_node_min_auto(line.to_string(), i/10);
                }
            }

            if !exists {

                let mut group = Group::new(
                    line.to_string(),
                    gen_radius(&coordinates, 0, radius*4),
                    util::gen_rgba(),
                    radius
                );

                group.new_node_min_auto(line.to_string(), i);

                groups.push(group);

            }

        }

        println!("{}",i);
    }

    groups
}