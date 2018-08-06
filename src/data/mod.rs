
use node::coordinates::*;
use tools::gen_rgba;
use std::collections::hash_map::DefaultHasher;
use std::fs::OpenOptions;
use std::hash::{Hash, Hasher};
use std::io::prelude::*;
use std::io;
use super::{Group, Coordinate};

/// Reads from the provided file, and converts to a path network using default settings.
pub fn convert_file<'a, 'b>(path: &str, lambda: &Fn(&str) -> bool) -> Result<Vec<Group<'a, 'b>>, io::Error> {
    let content = get_content(path)?;
    Ok(convert(content, &lambda))
}

/// Reads from the provided file, and returns content.
fn get_content(path: &str) -> Result<String, io::Error> {
    let mut file = OpenOptions::new()
        .read(true)
        .open(path)?;

    let mut contents = String::new();
    let _ = file.read_to_string(&mut contents);
    Ok(contents)
}

/// Initializes a CustomConverter a converts the content to a vector of groups and links.
pub fn convert<'a, 'b>(content: String, lambda: &Fn(&str) -> bool) -> Vec<Group<'a, 'b>> {
    let cct = CustomConverter::new('\n', 50, 50, &lambda);
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
        lambda_tag: &'a Fn(&str) -> bool)
        -> CustomConverter
    {
        CustomConverter {
            split,
            node_range,
            radius,
            size: 1000,
            lambda_tag,
            ignore_empty_lines: true,
        }
    }
}

/// Constructs a vector of groups and links using a CustomConverter and the string to analyze.
pub fn convert_inner<'a, 'b>(content: String, cct: CustomConverter) -> Vec<Group<'a,'b>> {
    let mut groups: Vec<Group> = Vec::new();

    let lines = content.split(cct.split);

    // Check if a group matches the same.
    // Stores the hashed array position rem.
    let coordinates = Coordinate::new(0, 0);
    let mut groups_boolean_array: [bool; 1000] = [false; 1000]; // TODO improve.

    for line in lines {
        // Ignore empty lines.
        if cct.ignore_empty_lines && line == "" {continue};

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
                    gen_radius(&coordinates, 0, cct.radius),
                );

                let len = group.nodes.len() as u32;
                group.new_node_min_auto("", cct.node_range); // TODO don't use empty str.
                group.settings.color = gen_rgba();
                groups.push(group);
            }

        }

    }
    groups
}

/// Calculates a default hash.
pub fn calculate_hash<T: Hash>(t: &T) -> u64 {
    let mut s = DefaultHasher::new();
    t.hash(&mut s);
    s.finish()
}
