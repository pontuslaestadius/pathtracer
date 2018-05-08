extern crate pathfinder;
extern crate rand;
extern crate image;
use pathfinder::*;
use image::Rgba;

/**
Creates two groups filled with a set of children randomly position within a radius.
The two groups are linked together.

**/


fn main() {

    // Coordinates for the groups.
    let group1_coordinates = Coordinate::new(0,0);
    let group2_coordinates = Coordinate::new(250,250);
    let group3_coordinates = Coordinate::new(150,20);

    // Names for the groups.
    let group1_name = "john doe".to_string();
    let group2_name = "jane doe".to_string();
    let group3_name = "sane moo".to_string();

    // Colors for the groups.
    let group1_color = Rgba {data: [250, 20, 20, 255]};
    let group2_color = Rgba {data: [20, 20, 250, 255]};
    let group3_color = Rgba {data: [20, 250, 20, 255]};



    let mut group2: Group<Square> = Group::new(
        &group2_name,
        group2_coordinates,
    );

    let mut group3 = Group::new(
        &group3_name,
        group3_coordinates,
    );

    let mut group1 = Group::new(
        &group1_name,
        group1_coordinates,
    );

    group1.settings.color = group1_color;
    group2.settings.color = group2_color;
    group3.settings.color = group3_color;

    group1.settings.radius = Some(50);
    group2.settings.radius = Some(70);
    group3.settings.radius = Some(40);

    // List of groups.
    //let mut groups: Vec<Group<Square>> = vec!(&group1, &group2, &group3);
    let children = 800;

    // Add children.
    map::network::add_children(&mut group1, children);
    map::network::add_children(&mut group2, children);
    map::network::add_children(&mut group3, children);

    //group1.link(&group2);

    // Where and what to call the file.
    let path= std::path::Path::new("random.png");
    let mut map = Map::new();

    map = map
        .map(&[group1, group2, group3]);

    let _ = map.image.unwrap().save(&path);
}


