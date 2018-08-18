
use coordinate::*;
use tools::gen_rgba;
use std::collections::hash_map::DefaultHasher;
use std::fs::OpenOptions;
use std::io::prelude::*;
use std::hash::{Hash, Hasher};
use std::io;
use super::{Group, Coordinate};

/// Holds configurations for converting a content String to a path network.
pub struct CustomConverter<'a> {
    pub split: char,
    pub node_range: u32,
    pub radius: u32,
    pub size: u64,
    pub lambda_tag: &'a Fn(&str) -> bool,
    pub ignore_empty_lines: bool,
}

/// Reads from the provided file, and converts to a path network using default settings.
pub fn convert_file<'a, 'b>(path: &str, lambda: &Fn(&str) -> bool) -> Result<Vec<Group>, io::Error> {
    let content = get_content(path)?;
    Ok(convert(&content, &lambda))
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
pub fn convert<'a, 'b>(content: &str, lambda: &Fn(&str) -> bool) -> Vec<Group> {
    let cct = CustomConverter::new('\n', 50, 50, &lambda);
    convert_inner(&content, &cct)
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
            size: 500,
            lambda_tag,
            ignore_empty_lines: true,
        }
    }
}

/// Constructs a vector of groups and links using a CustomConverter and the string to analyze.
pub fn convert_inner<'a, 'b>(content: &str, cct: &CustomConverter) -> Vec<Group> {
    let mut groups: Vec<Group> = Vec::new();

    let lines = content.split(cct.split);

    // Check if a group matches the same.
    // Stores the hashed array position rem.
    let coordinates = Coordinate::new(0, 0);
    let mut groups_boolean_array: [bool; 500] = [false; 500];

    for line in lines {
        // Ignore empty lines, if enabled. Or match the lambda tag to retrieve it.
        if (cct.ignore_empty_lines && line == "") || !(cct.lambda_tag)(line) {continue};

        // Hashes the input value for faster comparison.
        let hashed_line = calculate_hash(&line);

        // Checks the boolean array position for the groups existence.
        if groups_boolean_array[(hashed_line % cct.size) as usize] {

            for old in &mut groups.iter_mut() {
                // If it does not match existing tag.
                if old.settings.hash != hashed_line {continue};
                let _ = old.new_node_min_auto("", cct.node_range);
                break;
            }

            // Creates a new group because one did not exist.
        } else {
            // Sets the group to exists in the boolean array.
            groups_boolean_array[(hashed_line % cct.size) as usize] = true;
            // Produce a new group.
            let mut group = Group::new(
                &line,
                gen_radius(coordinates, 0, cct.radius),
            );

            group.new_node_min_auto("", cct.node_range);
            group.settings.color = gen_rgba();
            groups.push(group);
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;
    use std::io::prelude::*;
    use std::fs::File;
    use std::fs;

    fn eval_result(res: Vec<Group>) {
        assert_eq!(res.len(), 3);
        assert_eq!(res[0].nodes.len(), 2);
        assert_eq!(res[1].nodes.len(), 5);
        assert_eq!(res[2].nodes.len(), 3);
    }

    #[test]
    fn test_convert_inner() {
        let cct = CustomConverter {
            split: ' ',
            node_range: 10,
            radius: 50,
            size: 50,
            lambda_tag: &|_x| true,
            ignore_empty_lines: true,
        };

        let content = "a b c a b c b b b c";
        let res = convert_inner(content, &cct);
        eval_result(res);
    }

    #[test]
    fn test_convert() {
        let content = "a\nb\nc\na\nb\nc\nb\nb\nb\nc";
        let res = convert(content, &|_x| true);
        eval_result(res);
    }

    #[test]
    fn test_convert_file() {
        let path = Path::new("test.txt");
        let mut file = File::create(&path).unwrap();

        let content = "a\nb\nc\na\nb\nc\nb\nb\nb\nc";
        let _ = file.write_all(content.as_bytes()).unwrap();
        let res = convert_file("test.txt", &|_x| true).unwrap();
        eval_result(res);
        let _ = fs::remove_file("test.txt").unwrap();
    }
}

