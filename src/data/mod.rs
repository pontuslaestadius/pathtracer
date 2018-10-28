use super::{Coordinate, Group};
use coordinate::*;
use std::{
    collections::hash_map::DefaultHasher,
    fs::OpenOptions,
    hash::{Hash, Hasher},
    io::{self, prelude::*},
};
use tools::gen_rgba;

/// Holds configurations for converting a content String to a path network.
pub struct CustomConverter<'a> {
    pub split: char,
    pub node_range: u32,
    pub radius: u32,
    pub size: u64,
    pub lambda_tag: &'a Fn(&str) -> bool,
    pub link_groups: bool,
    pub ignore_empty_lines: bool,
}

/// Reads from the provided file, and converts to a path network using default
/// settings.
pub fn convert_file(path: &str, lambda: &Fn(&str) -> bool) -> Result<Vec<Group>, io::Error> {
    let content = get_content(path)?;
    Ok(convert(&content, &lambda))
}

/// Reads from the provided file, and returns content.
fn get_content(path: &str) -> Result<String, io::Error> {
    let mut file = OpenOptions::new().read(true).open(path)?;

    let mut contents = String::new();
    let _ = file.read_to_string(&mut contents);
    Ok(contents)
}

/// Initializes a CustomConverter a converts the content to a vector of groups
/// and links.
pub fn convert(content: &str, lambda: &Fn(&str) -> bool) -> Vec<Group> {
    let cct = CustomConverter::new('\n', 120, 120, &lambda);
    convert_inner(&content, &cct)
}

impl<'a> CustomConverter<'a> {
    /// Constructs a new CustomConverter configuration for data interpretation
    /// for a path network.
    pub fn new(
        split: char,
        node_range: u32,
        radius: u32,
        lambda_tag: &'a Fn(&str) -> bool,
    ) -> CustomConverter {
        CustomConverter {
            split,
            node_range,
            radius,
            size: 500,
            lambda_tag,
            ignore_empty_lines: true,
            link_groups: true,
        }
    }
}

/// Constructs a vector of groups and links using a CustomConverter and the
/// string to analyze.
pub fn convert_inner(content: &str, cct: &CustomConverter) -> Vec<Group> {
    let mut groups: Vec<Group> = Vec::new();
    let lines = content.split(cct.split);
    let coordinates = Coordinate::new(0, 0);
    let mut groups_boolean_array: [bool; 500] = [false; 500];

    for line in lines {
        // Ignore empty lines, if enabled. Or match the lambda tag to retrieve it.
        if (cct.ignore_empty_lines && line == "") || !(cct.lambda_tag)(line) {
            continue;
        };

        let hashed_line = calculate_hash(&line);
        // Checks the boolean array position for the groups existence.
        if groups_boolean_array[(hashed_line % cct.size) as usize] {
            // Add a new node to the existing group.
            let index = groups
                .iter()
                .position(|ref g| g.settings.hash == hashed_line)
                .expect("Group located, but no hash matching.");
            groups[index].new_node_min_max(index as u32, cct.node_range);

        // Creates a new group because one did not exist.
        } else {
            // Sets the group to exists in the boolean array.
            groups_boolean_array[(hashed_line % cct.size) as usize] = true;
            let mut group = Group::new(&line, gen_radius(coordinates, 0, cct.radius));
            group.settings.color = gen_rgba();

            if cct.link_groups && !groups.is_empty() {
                let tmp = &groups[groups.len() - 1];
                let n = if tmp.nodes.is_empty() {
                    &tmp.settings
                } else {
                    &tmp.nodes[tmp.nodes.len() - 1]
                };

                group.settings.link(n);
            }

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
    use super::{super::Draw, *};
    use std::{
        fs::{self, File},
        path::Path,
    };

    fn eval_result(res: Vec<Group>) {
        assert_eq!(res.len(), 3);
        assert_eq!(res[0].nodes.len(), 1);
        assert_eq!(res[1].nodes.len(), 4);
        assert_eq!(res[2].nodes.len(), 2);
    }

    #[test]
    fn test_convert_inner() {
        let cct = CustomConverter {
            split: '-',
            node_range: 10,
            radius: 50,
            size: 50,
            lambda_tag: &|_x| true,
            ignore_empty_lines: true,
            link_groups: true,
        };

        let content = "a-b-c-a-b-c-b--b-b-c";
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

    #[test]
    fn test_link_groups() {
        let content = "a\nb\nc\na\nb\nc\nb\nb\nb\nc";
        let res = convert(content, &|_x| true);

        for (i, g) in res.iter().enumerate().rev() {
            if i == 0 {
                break;
            }
            let left = g.get_links()[0].t;
            assert_ne!(
                left,
                0,
                "Result did not link forward. ({:?})",
                g.get_links()
            );
        }
    }
}
