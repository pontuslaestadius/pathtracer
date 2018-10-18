//! Author: Pontus Laestadius
//! Version: 0.3
//! Since: 2017-12-02
//!
//! Visualizes a provided log with the marked tags.

extern crate pathfinder;
extern crate rand;
use pathfinder::{data, group, *};
use std::{env, path::Path};

fn main() {
    // Gets command line arguments.
    let args: Vec<String> = env::args().collect();

    // If no arguments provided. Notify user and exit.
    if args.len() < 3 {
        println!("Invalid arguments, application requires: <input> <output> [tag]");
        std::process::exit(1);
    }

    // The tag to find to group them by.
    let find: &str = if args.len() > 3 {
        args[3].as_str()
    } else {
        "Author"
    };

    let lambda = |x: &str| x.starts_with(find);

    // Fetches the log, from the command line argument.
    let log = &args[1].as_str();

    // Use the log directory and the tag to create the groups.
    let groups: Vec<Group> = data::convert_file(log, &lambda).unwrap();
    let n = group::count(&groups);
    println!("{:?} groups with {} nodes", groups.len(), n);

    let map = Map::new();
    map.map(&groups).save(Path::new(&args[2])).unwrap();
}
