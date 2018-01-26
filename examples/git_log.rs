//! Author: Pontus Laestadius
//! Version: 0.1
//! Since: 2017-12-02
//!
//! Visualizes a provided log with the marked tags.
//!

extern crate pathfinder;
use pathfinder::{node, map, data, group};

fn main() {
    // The tag to find to group them by.
    let find: String = "Author".to_string();

    let nothing = Vec::new();

    // Save the tag to use to convert the data.
    let tag = data::Tag {collection: find, ignore: nothing};

    // Fetches the log, replace it with your log directory.
    let log = "resources/log.txt";

    // Use the log directory and the tag to create the groups.
    let (groups, links) = data::convert_file(log, &tag);

    // Count the groups and nodes.
    let (g, n) = group::count(&groups);

    // Print them.
    println!("{:?} groups with {} nodes", g, n);

    // Save path for the final result.
    let save_path = "visual_data.png";

    // Map them to an RGBA Image and saves it.
    map::groups_and_links(&groups, &links, save_path);
}