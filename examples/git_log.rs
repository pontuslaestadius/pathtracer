/*
    Visualizes a provided log with the marked tags.
*/

extern crate pathfinder;
extern crate rand;
use pathfinder::{data, group, *};
use std::{env, path::Path};

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        panic!("Invalid arguments, application requires: <input> <output> [tag]");
    }

    let find: &str = if args.len() > 3 { &args[3] } else { "Author" };

    let lambda = |x: &str| x.starts_with(find);
    let groups: Vec<Group> = data::convert_file(&args[1], &lambda)?;
    let n = group::count(&groups);
    println!("{:?} groups with {} nodes", groups.len(), n);
    Map::new().map(&groups).save(Path::new(&args[2]))
}
