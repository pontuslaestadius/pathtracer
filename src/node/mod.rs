use super::Node;
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_node_names_fail() {
        let res = get_node_names("invalid path");
        assert_eq!(res.is_err(), true);
    }

}
