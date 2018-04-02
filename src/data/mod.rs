use group::*;
use node::coordinates::*;
use std::collections::hash_map::DefaultHasher;
use std::fs::OpenOptions;
use std::hash::{Hash, Hasher};
use std::io::prelude::*;
use super::node::link::*;
use super::tools::util;

/// Reads from the provided file, and converts to a path network using default settings.
pub fn convert_file<'a>(path: &str, lambda: &Fn(&str) -> bool) -> (Vec<Group>, Vec<Link<'a>>) {
    let mut file = OpenOptions::new()
        .read(true)
        .open(path)
        .unwrap();

    let mut contents = String::new();
    let _ = file.read_to_string(&mut contents);

    convert(contents, &lambda)
}

/// Initializes a CustomConverter a converts the content to a vector of groups and links.
pub fn convert<'a>(content: String, lambda: &Fn(&str) -> bool) -> (Vec<Group>, Vec<Link<'a>>) {
    let cct = CustomConverter::new('\n', 100, 50, 1000, &lambda);
    convert_inner(content, cct)
}

/// Holds configurations for converting a content String to a path network.
pub struct CustomConverter<'a> {
    split: char,
    node_range: u32,
    radius: u32,
    size: u64,
    lambda_tag: &'a Fn(&str) -> bool,
    ignore_empty_lines: bool,
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

    /// Sets the 'split' character for interpreting between lines.
    pub fn set_split(&mut self, property: char) {
        self.split = property;
    }

    /// Sets the node_range for setting the range a node can spawn inside a Group.
    pub fn set_node_range(&mut self, property: u32) {
        self.node_range = property;
    }

    /// Set the radius a group is able to spread it's nodes.
    pub fn set_radius(&mut self, property: u32) {
        self.radius = property
    }

    /// Set the amount of Groups the hash table holds.
    pub fn set_size(&mut self, property: u64) {
        self.size = property;
    }

    /// Set the function used to differentiate between groups.
    pub fn set_lambda_tag(&mut self, property: &'a Fn(&str) -> bool) {
        self.lambda_tag = property;
    }

    /// Set to ignore empty lines. Defaults to true.
    pub fn set_ignore_empty_lines(&mut self, property: bool) {
        self.ignore_empty_lines = property;
    }

}


/// Constructs a vector of groups and links using a CustomConverter and the string to analyze.
pub fn convert_inner<'a>(content: String, cct: CustomConverter) -> (Vec<Group>, Vec<Link<'a>>) {
    let mut groups: Vec<Group> = Vec::new();

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

// TODO MAKE PRIVATE
/// Calculates a default hash.
pub fn calculate_hash<T: Hash>(t: &T) -> u64 {
    let mut s = DefaultHasher::new();
    t.hash(&mut s);
    s.finish()
}