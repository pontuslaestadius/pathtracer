extern crate image;
extern crate pathtracer;
extern crate rand;

use pathtracer::*;

use std::path::Path;

fn main() -> std::io::Result<()> {
    let mut groups = Vec::new();
    let coordinates = Shape::Square.area(10);
    let children: u32 = 100;
    let radius: u32 = 20;
    let spread = 60;

    let len = coordinates.len();

    println!(
        "{} Groups will be rendered. {} Nodes",
        len,
        len * children as usize
    );

    for (i, c) in coordinates.iter().enumerate() {
        let mut group = cluster!(c.x * spread, c.y * spread);
        group.radius(radius);
        group.color(tools::seed_rgba((i * 23) as u64));
        group.add(children);
        groups.push(group);
    }
    Map::new().map(&groups).save(&Path::new("out.jpg"))
}
