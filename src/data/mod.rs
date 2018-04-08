
use node::coordinates::*;
use std::collections::hash_map::DefaultHasher;
use std::fs::OpenOptions;
use std::hash::{Hash, Hasher};
use std::io::prelude::*;
use super::{Group, Coordinate, Shape, Link};

/// Reads from the provided file, and converts to a path network using default settings.
pub fn convert_file<'a, T: Shape>(path: &str, lambda: &Fn(&str) -> bool) -> (Vec<Group<T>>, Vec<Link<'a>>) {
    let content = get_content(path);
    convert(content, &lambda)
}


/// Reads from the provided file, and returns content.
fn get_content(path: &str) -> String {
    let mut file = OpenOptions::new()
        .read(true)
        .open(path)
        .unwrap();

    let mut contents = String::new();
    let _ = file.read_to_string(&mut contents);
    contents
}

/// Initializes a CustomConverter a converts the content to a vector of groups and links.
pub fn convert<'a, T: Shape>(content: String, lambda: &Fn(&str) -> bool) -> (Vec<Group<T>>, Vec<Link<'a>>) {
    let cct = CustomConverter::new('\n', 100, 50, 1000, &lambda);
    convert_inner(content, cct)
}

/// Holds configurations for converting a content String to a path network.
pub struct CustomConverter<'a> {
    pub split: char,
    pub node_range: u32,
    pub radius: u32,
    pub size: u64,
    pub lambda_tag: &'a Fn(&str) -> bool,
    pub ignore_empty_lines: bool,
}

impl<'a> CustomConverter<'a> {

    /// Constructs a new CustomConverter configuration for data interpretation for a path network.
    pub fn new(
        split: char,
        node_range: u32,
        radius: u32,
        size: u64,
        lambda_tag: &'a Fn(&str) -> bool)
        -> CustomConverter
    {
        CustomConverter {
            split,
            node_range,
            radius,
            size,
            lambda_tag,
            ignore_empty_lines: true,
        }
    }
}

/// Constructs a vector of groups and links using a CustomConverter and the string to analyze.
pub fn convert_inner<'a, T: Shape>(content: String, cct: CustomConverter) -> (Vec<Group<T>>, Vec<Link<'a>>) {
    let mut groups: Vec<Group<T>> = Vec::new();

    let lines = content.split(cct.split);

    // Check if a group matches the same.
    // Stores the hashed array position rem.
    let coordinates = Coordinate::new(0, 0);
    let mut groups_boolean_array: [bool; 1000] = [false; 1000]; // TODO improve.

    for line in lines {
        // Ignore empty lines.
        if cct.ignore_empty_lines {
            if line == "" {continue};
        }

        // Pick up tagged lines.
        if (cct.lambda_tag)(line) {

            // Hashes the input value for faster comparison.
            let hashed_line = calculate_hash(&line);

            // Checks the boolean array position for the groups existence.
            if groups_boolean_array[(hashed_line % cct.size) as usize] {

                for old in &mut groups.iter_mut() {
                    // If it does not match existing tag.
                    if old.settings.hash != hashed_line {continue};
                    let _ = old.new_node_min_auto("", cct.node_range); // TODO don't use empty str.
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
                );
                group.settings.radius = Some(cct.radius/4);

                let len = group.nodes.len() as u32;
                group.new_node_min_auto("", len); // TODO don't use empty str.
                groups.push(group);
            }

        }

    }

    (groups, Vec::new())
}

/// Calculates a default hash.
pub fn calculate_hash<T: Hash>(t: &T) -> u64 {
    let mut s = DefaultHasher::new();
    t.hash(&mut s);
    s.finish()
}