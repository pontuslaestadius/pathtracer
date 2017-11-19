
use super::group::*;
use super::super::pathfinder::node::coordinates::*;
use super::super::pathfinder::node::Node;
use super::super::pathfinder::network::gen_rgba;

use std::fs::{OpenOptions, File};
use std::io::prelude::*;
use std::io;


pub struct Tag {
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
    let lines = content.split("\n");

    let mut groups: Vec<Group> = Vec::new();

    let mut ignore_line = false;
    for line in lines {
        // Ignore empty lines.
        if line == "" {continue};

        if line.starts_with(tag.collection.as_str()) {

            // Check if a group matches the same.
            let coordinates = Coordinates::new(0,0);
            let radius = 40;

            let mut exists = false;
            for mut old in &mut groups {
                if old.name == line {
                    exists = true;
                    old.new_node(line.to_string());

                }
            }

            if !exists {

                let mut group = Group::new(
                    line.to_string(),
                    Coordinates::gen_within_radius(&coordinates, radius*4),
                    gen_rgba(),
                    radius
                );

                groups.push(group);

            }

        }

    }

    groups
}