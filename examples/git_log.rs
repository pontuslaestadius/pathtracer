//! Author: Pontus Laestadius
//! Version: 0.3
//! Since: 2017-12-02
//!
//! Visualizes a provided log with the marked tags.
//!

extern crate pathfinder;
extern crate rand;
use pathfinder::{data, group};
use pathfinder::*;
use std::env;

fn main() {

    // Gets command line arguments.
    let args: Vec<String> = env::args().collect();

    // If no arguments provided. Notify user and exit.
    if args.len() < 3 {
        println!("Invalid arguments, application requires: \
        <input> <output> [tag]");
        return ();
    }

    // The tag to find to group them by.
    let find: String = if args.len() > 3 {
        args[3].to_string()
    } else {
        "Author".to_string()
    };

    let lambda = |x: &str| {
        x.starts_with(find.as_str())
    };

    // Fetches the log, from the command line argument.
    let log = &args[1].as_str();

    // Use the log directory and the tag to create the groups.
    let groups:Vec<Group> = data::convert_file(log, &lambda).unwrap();

    // Count the groups and nodes.
    let (g, n) = group::count(&groups);

    // Print them.
    println!("{:?} groups with {} nodes", g, n);

    // Save path for the final result.
    let path = std::path::Path::new(&args[2]);

    let mut map = Map::new();
    map = map
        .map(&groups);
    let _ = map.image.unwrap().save(&path);
}
