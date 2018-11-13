use super::*;
use std::{
    cmp::PartialEq,
    fs::File,
    io::{self, prelude::*},
};

impl<'a> PartialEq for Node {
    fn eq(&self, other: &Node) -> bool { self.hash == other.hash }
}

/// Returns a list of Strings split using \n in a Vec.
pub fn get_node_names(path: &str) -> Result<Vec<String>, io::Error> {
    let mut file = File::open(path)?;
    let mut contents = String::new();
    let mut names: Vec<String> = Vec::new();

    file.read_to_string(&mut contents)?;
    let split = contents.split('\n');
    for value in split {
        names.push(value.to_string());
    }
    Ok(names)
}

/// Prints the distance between all the nodes paths and returns a summary of
/// the total distance.
pub fn path_print(path: &[Node]) -> u32 { verbose_path(path, true) }

/// Returns the sum distance that all the nodes' are from each other.
pub fn path_distances(path: &[Node]) -> u32 { verbose_path(path, false) }

/// Implementation of path_distance and path_print, Use those for interfacing.
fn verbose_path(path: &[Node], side_effects: bool) -> u32 {
    let mut distance = 0;
    let mut prev = Coordinate::new(0, 0);
    for (link_i, leg) in path.iter().enumerate() {
        let dis = coordinate::distance(prev, leg.position());
        distance += dis;
        prev = leg.position();
        if side_effects {
            println!("#{} {:?} - distance: {}", link_i, leg.position(), dis);
        }
    }

    if side_effects {
        println!("Total distance: {}", distance);
    }
    distance
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_node_names_fail() {
        let res = get_node_names("invalid path");
        assert!(res.is_err());
    }

}
