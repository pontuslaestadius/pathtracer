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
    let circle = shape::Circle::new();
    let coordinates = circle.area(7);
    let children: u32 = 20;
    let radius = 10;
    let spread = 18;

    let len = coordinates.len();

    println!(
        "{} Groups will be rendered. {} Nodes",
        len,
        len * children as usize
    );

    for (i, c) in coordinates.iter().enumerate() {
        let mut group = Group::new_simple(c.x * spread, c.y * spread);
        group.radius(radius as u32);
        group.color(tools::seed_rgba((i * 100) as u64));
        map::network::add_children(&mut group, children);
        groups.push(group);
    }
    Map::new().map(&groups).save(&Path::new("out.jpg"))
}
