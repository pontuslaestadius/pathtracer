extern crate image;
extern crate pathfinder;
extern crate rand;
use pathfinder::*;
use std::path::Path;

/*
    Creates three groups filled with children randomly position within it's radius.
    These numbers may need to be lowered depending on the machine.
*/

fn main() -> std::io::Result<()> {
    let mut groups = Vec::new();
    let coordinates = shape::Circle::new().area(30);
    let children: u32 = 10;
    let radius: u32 = 5;
    let spread = 5;

    let len = coordinates.len();

    println!(
        "{} Groups will be rendered. {} Nodes",
        len,
        len * children as usize
    );

    for (i, c) in coordinates.iter().enumerate() {
        let mut group = Group::new_simple(c.x * spread, c.y * spread);
        group.radius(radius);
        group.color(tools::seed_rgba((i * 70) as u64));
        map::network::add_children(&mut group, children);
        groups.push(group);
    }
    Map::new().map(&groups).save(&Path::new("out.jpg"))
}
