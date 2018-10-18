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
    let coordinates = circle.area(15);
    let children: u32 = 100;
    let radius = 7;

    let len = coordinates.len();
    let d = 255 as f64 / len as f64;
    let mut col = [0f64; 3];

    println!(
        "{} Groups will be rendered. {} Nodes",
        len,
        len * children as usize
    );

    for c in coordinates.iter() {
        let mut group = Group::new_simple(c.x * radius, c.y * radius);
        group.settings.radius = Some(radius as u32);

        for i in 0..col.len() {
            if col[i] as u8 != 255 {
                col[i] += d;
                break;
            }
        }

        group.set_color(col[0] as u8, col[1] as u8, col[2] as u8);
        map::network::add_children(&mut group, children);
        groups.push(group);
    }
    let _ = Map::new().map(&groups).save(&Path::new("out.png")).unwrap();
}
