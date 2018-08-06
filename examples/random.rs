extern crate pathfinder;
extern crate rand;
extern crate image;
use pathfinder::*;
use image::Rgba;

/*
    Creates three groups filled with children randomly position within it's radius.
*/

fn main() { 
    let children = 800;
    let color = [
        Rgba {data: [250, 20, 20, 255]},
        Rgba {data: [20, 20, 250, 255]},
        Rgba {data: [20, 250, 20, 255]}
    ];
    let radius = [
        Some(50),
        Some(70),
        Some(40)
    ];

    let mut groups = Group::from_list(&[(0,0),(250,250),(150,20)]);

    for (i, ref mut group) in groups.iter_mut().enumerate() {
        group.settings.radius = radius[i];
        group.settings.color = color[i];
        map::network::add_children(group, children);
    }

    // Where and what to call the file.
    let path = std::path::Path::new("out.png");
    let mut map = Map::new();
    map = map.map(&groups);
    let _ = map.image.unwrap().save(&path);
}

