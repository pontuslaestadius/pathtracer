//! Author: Pontus Laestadius
//! Version: 0.3
//! Since: 2017-12-02
//!
//! Visualizes a provided log with the marked tags.
//!

extern crate pathfinder;
extern crate rand;
use pathfinder::{map, data, group, node};
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
    let (groups, links): (Vec<Group<Triangle>>, Vec<node::link::Link>) = data::convert_file(log, &lambda);

    // Count the groups and nodes.
    let (g, n) = group::count(&groups);

    // Print them.
    println!("{:?} groups with {} nodes with {} links", g, n, links.len());

    // Save path for the final result.
    let path = std::path::Path::new(&args[2]);

    // Map them to an RGBA Image and saves it.
    map::groups_and_links(&path, &groups, &links);
}