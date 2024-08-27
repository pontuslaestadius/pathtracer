/*!
Extra functionality for Nodes.
*/

use super::*;
use std::{
    cmp::PartialEq,
    fs::File,
    io::{self, prelude::*},
};

impl<'a> PartialEq for Node {
    fn eq(&self, other: &Node) -> bool {
        self.hash == other.hash
    }
}

/**
Returns a list of Strings split using \n in a Vec.


## Errors

Could not open file.
Could not read content to string.
*/
pub fn get_node_names(path: &str) -> Result<Vec<String>, io::Error> {
    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents.split('\n').fold(vec![], |mut acc, x| {
        acc.push(x.to_string());
        acc
    }))
}

/**
Write the positions to a file.


## Errors

Could not open file.
Could not write to file.
 */
pub fn write_file(path: &str, nodes: &[Node]) -> std::io::Result<()> {
    let mut file = File::create(path)?;
    for n in nodes.iter() {
        let p = n.position();
        writeln!(&mut file, "{},{}", p.x, p.y)?;
    }
    Ok(())
}

/**
Reads the positions from a file.


Expects eact line format to be:

x,y


## Errors

Could not open file.
Could not read to string.

*/
pub fn from_file(path: &str) -> Result<Vec<Node>, io::Error> {
    let mut contents = String::new();
    let mut file = File::open(path)?;
    file.read_to_string(&mut contents)?;
    Ok(contents.split('\n').fold(vec![], |mut acc, x| {
        // Ignore empty lines, mostly occurs as file's tend to end with an empty line.
        // Not that the protocal supports --- value, <blank line>, value --- explicity.
        if x.is_empty() {
            return acc;
        }
        let vals = x.split(',').collect::<Vec<_>>();
        if vals.len() != 2 {
            panic!(
                "Failed to decode values from file: '{}', invalid line: '{}'",
                path, x
            );
        }
        let c = Coordinate::new(
            vals[0].parse::<i16>().unwrap(),
            vals[1].parse::<i16>().unwrap(),
        );
        acc.push(Node::new(x, c));
        acc
    }))
}

/**
Prints the distance between all the nodes paths and returns a summary of the total distance.
*/
pub fn path_print(path: &[Node]) -> u32 {
    verbose_path(path, true)
}

/**
 Returns the sum distance that all the nodes' are from each other.
*/
pub fn path_distances(path: &[Node]) -> u32 {
    verbose_path(path, false)
}

/**
Implementation of path_distance and path_print, Use those for interfacing.
*/
fn verbose_path<L: Location>(path: &[L], stdout: bool) -> u32 {
    let mut prev = Coordinate::new(0, 0);
    let distance = path.iter().fold(0, |sum, x| {
        let dis = coordinate::distance(prev, x.position());
        prev = x.position();
        if stdout {
            debug!("{} - distance: {}", x.position(), dis);
        }
        sum + dis
    });

    if stdout {
        debug!("Total distance: {}", distance);
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
