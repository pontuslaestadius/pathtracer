/*!
Converts strings to mapped Nodes and Groups.
*/

use super::{consts, coordinate, tools, Coordinate, Group};
use std::{
    collections::hash_map::DefaultHasher,
    fs::OpenOptions,
    hash::{Hash, Hasher},
    io::{self, prelude::*},
};

/**
Holds configurations for converting a content String to a path network.
*/
pub struct CustomConverter<'a> {
    pub split: char,
    pub node_range: u32,
    pub radius: u32,
    pub lambda_tag: &'a dyn Fn(&str) -> bool,
    pub link_groups: bool,
}

/**
Reads from the provided file, and converts to a path network using default settings.
*/
pub fn convert_file(path: &str, lambda: &dyn Fn(&str) -> bool) -> Result<Vec<Group>, io::Error> {
    let content = content(path)?;
    Ok(convert(&content, &lambda))
}

/**
Reads from the provided file, and returns content.
*/
fn content(path: &str) -> Result<String, io::Error> {
    let mut file = OpenOptions::new().read(true).open(path)?;

    let mut contents = String::new();
    let _ = file.read_to_string(&mut contents);
    Ok(contents)
}

/**
Initializes a CustomConverter a converts the content to a vector of groups and links.
*/
pub fn convert(content: &str, lambda: &dyn Fn(&str) -> bool) -> Vec<Group> {
    let cct = CustomConverter::new('\n', 30, 120, &lambda);
    convert_inner(&content, &cct).unwrap()
}

impl<'a> CustomConverter<'a> {
    /**
    Constructs a new CustomConverter configuration for data interpretation for a path network.
    */
    pub fn new(
        split: char,
        node_range: u32,
        radius: u32,
        lambda_tag: &'a dyn Fn(&str) -> bool,
    ) -> CustomConverter {
        CustomConverter {
            split,
            node_range,
            radius,
            lambda_tag,
            link_groups: true,
        }
    }
}

/**
Constructs a vector of groups and links using a CustomConverter and the string to analyze.
*/
pub fn convert_inner(content: &str, cct: &CustomConverter) -> io::Result<Vec<Group>> {
    let mut gr_bool_arr: [bool; consts::NETWORK_REM] = [false; consts::NETWORK_REM];

    let lines = content
        .split(cct.split)
        .filter(|x| !x.is_empty() && (cct.lambda_tag)(x))
        .collect::<Vec<_>>();

    let lines = lines.iter().fold(vec![], |acc, hash| {
        let hash = calculate_hash(hash);
        let pos = (hash % consts::NETWORK_REM as u64) as usize;
        if !gr_bool_arr[pos] {
            gr_bool_arr[pos] = true;
            push_group(acc, hash)
        } else {
            push_node(acc, hash)
        }
    });
    Ok(lines)
}

fn push_group(mut groups: Vec<Group>, hash: u64) -> Vec<Group> {
    let mut group = Group::new("", coordinate::gen_radius(Coordinate::new(1, 0), 0, 100));
    group.settings.hash = hash;
    group.settings.color = tools::seed_rgba(hash);
    group.new_node_min_max(groups.len() as u32, 40);
    if !groups.is_empty() {
        group.nodes[0].link(groups.last().unwrap().nodes.last().unwrap());
    }
    groups.push(group);
    groups
}

fn push_node(mut groups: Vec<Group>, hash: u64) -> Vec<Group> {
    let index = groups
        .iter()
        .position(|ref g| g.settings.hash == hash)
        .expect("Group located, but no hash matching.");
    groups[index].new_node_min_max(index as u32, 40);
    groups
}

/**
Calculates a default hash.
*/
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
        assert_eq!(res[0].nodes.len(), 2);
        assert_eq!(res[1].nodes.len(), 5);
        assert_eq!(res[2].nodes.len(), 3);
    }

    #[test]
    fn test_convert_inner() {
        let cct = CustomConverter {
            split: '-',
            node_range: 10,
            radius: 50,
            lambda_tag: &|_x| true,
            link_groups: true,
        };

        let content = "a-b-c-a-b-c-b--b-b-c";
        let res = convert_inner(content, &cct).unwrap();
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
            let left = g.nodes[0].hl(0).unwrap().t;
            assert_ne!(left, 0, "Result did not link forward. ({:?})", g.links());
        }
    }
}
