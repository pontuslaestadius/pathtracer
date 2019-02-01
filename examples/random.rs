extern crate image;
extern crate pathfinder;
extern crate rand;
use pathfinder::*;
use std::path::Path;

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
        let mut group = cluster!(c.x * spread, c.y * spread);
        group.radius(radius);
        group.color(tools::seed_rgba((i * 70) as u64));
        group.add(children);
        groups.push(group);
    }
    Map::new().map(&groups).save(&Path::new("out.jpg"))
}
