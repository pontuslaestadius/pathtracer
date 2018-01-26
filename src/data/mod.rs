use group::*;
use node::coordinates::*;
use node::Node;

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
    let mut links: Vec<Link> = Vec::new();
    let lines = content.split("\n");

    let mut i = 0;
    let mut previous_tag = String::new();
    for line in lines {
        // Ignore empty lines.
        if line == "" {continue};

        // Pick up tagged lines.
        if line.starts_with(tag.collection.as_str()) {
            i+=1;

            // Check if a group matches the same.
            let coordinates = Coordinate::new(0, 0);
            let radius = 80;

            let mut exists = false;
            for old in &mut groups {
                // If it does not match existing tag.
                if old.name != line {continue};
                exists = true;

                let ref_node: &Node = old.new_node_min_auto(line.to_string(), (i/10));
                // Draw a line between the previous and the next commit if they are chained.
                /*
                if (old.name.clone() == previous_tag) {
                    links.push(Link::new(&old.geo, &ref_node.geo));
                }
                previous_tag = old.name;
                */
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

    }

    (groups, links)
}