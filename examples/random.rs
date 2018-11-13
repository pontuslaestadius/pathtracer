extern crate image;
extern crate pathfinder;
extern crate rand;
use pathfinder::*;
use std::path::Path;

/*
    Creates three groups filled with children randomly position within it's radius.
    These numbers may need to be lowered depending on the machine.
*/

fn main() {
    let mut groups = Vec::new();
    let circle = shape::Circle::new();
    let coordinates = circle.area(10);
    let children: u32 = 20;
    let radius = 10;
    let spread = 18;

    let len = coordinates.len();

    println!(
        "{} Groups will be rendered. {} Nodes",
        len,
        len * children as usize
    );

    for c in coordinates.iter() {
        let mut group = Group::new_simple(c.x * spread, c.y * spread);
        group.settings.radius = Some(radius as u32);
        group.settings.color = tools::gen_rgba();
        map::network::add_children(&mut group, children);
        groups.push(group);
    }
    let _ = Map::new().map(&groups).save(&Path::new("out.jpg")).unwrap();
}
