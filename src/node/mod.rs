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
    file.read_to_string(&mut contents)?;
    Ok(contents.split('\n').fold(vec![], |mut acc, x| {
        acc.push(x.to_string());
        acc
    }))
}

pub fn write_file(path: &str, nodes: &[Node]) -> std::io::Result<()> {
    let mut file = File::create(path)?;
    for n in nodes.iter() {
        let p = n.position();
        writeln!(&mut file, "{},{}", p.x, p.y)?;
    }
    Ok(())
}

pub fn from_file(path: &str) -> Result<Vec<Node>, io::Error> {
    let mut contents = String::new();
    let mut file = File::open(path)?;
    file.read_to_string(&mut contents)?;
    Ok(contents.split('\n').fold(vec![], |mut acc, x| {
        let vals = x.split(',').collect::<Vec<_>>();
        let c = Coordinate::new(
            vals[0].parse::<i16>().unwrap(),
            vals[1].parse::<i16>().unwrap(),
        );
        acc.push(Node::new(x, c));
        acc
    }))
}

/// Prints the distance between all the nodes paths and returns a summary of
/// the total distance.
pub fn path_print(path: &[Node]) -> u32 { verbose_path(path, true) }

/// Returns the sum distance that all the nodes' are from each other.
pub fn path_distances(path: &[Node]) -> u32 { verbose_path(path, false) }

/// Implementation of path_distance and path_print, Use those for interfacing.
fn verbose_path<L: Location>(path: &[L], side_effects: bool) -> u32 {
    let mut prev = Coordinate::new(0, 0);
    let distance = path.iter().fold(0, |sum, x| {
        let dis = coordinate::distance(prev, x.position());
        prev = x.position();
        if side_effects {
            println!("{} - distance: {}", x.position(), dis);
        }
        sum + dis
    });

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
