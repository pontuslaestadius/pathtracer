extern crate pathfinder;
extern crate rand;
extern crate image;
use pathfinder::*;
use image::Rgba;

/*
    Creates three groups filled with children randomly position within it's radius.
*/

fn main() {
    let children = 600;
    let color = [
        Rgba {data: [250, 20, 20, 255]},
        Rgba {data: [20, 20, 250, 255]},
        Rgba {data: [20, 250, 20, 255]}
    ];
    let radius = [
        Some(50),
        Some(60),
        Some(65)
    ];

    let mut groups = Group::from_list(&[(0,0),(240,20),(115,40)]);

    for (i, ref mut group) in groups.iter_mut().enumerate() {
        group.settings.radius = radius[i];
        group.settings.color = color[i];
        map::network::add_children(group, children);
    }

    // Where and what to call the file.
    let path = std::path::Path::new("out.png");
    let mut map = Map::new();
    map = map.map(&groups);
    let _ = map.save(&path).unwrap();
}

