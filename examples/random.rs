/*
    Store all nodes within a file.
        -> inside a subdirectory
       (->) If to many nodes exist use a secondary file.


    ( Node generation should be a secondary project, as they are too complex to hand write.)
    A node contains:
        -> name: &str
        -> connecting nodes
            -> distance to them
            -> ways of transportation with times.
       (->) absolute x and y positions.



    It will use multiple algorithms to find the shortest and fastest path using whatever
    transportation the user has requested. (any transportation by default)
-
    Djikstras, A-pointer
        (->) including alternative paths.

    The user will get a summery of the trip,
    The user can request a generated map of traveling between the nodes to the destination.

 */

extern crate pathfinder;
extern crate rand;
extern crate image;
use pathfinder::map;
use pathfinder::*;
use image::Rgba;

/**
Creates two groups filled with a set of children randomly position withined a radius.
The two groups are linked together.

**/

fn main() {
    // Coordinates for the groups.
    let group1_coordinates = Coordinate::new(0,0);
    let group2_coordinates = Coordinate::new(250,250);

    // Names for the groups.
    let group1_name = "john doe".to_string();
    let group2_name = "jane doe".to_string();

    // Colors for the groups.
    let group1_color = Rgba {data: [250, 20, 20, 255]};
    let group2_color = Rgba {data: [20, 20, 250, 255]};

    // Group 1.
    let mut group1 = Group::new(
        &group1_name,
        group1_coordinates,
    );

    // Group 2.
    let mut group2 = Group::new(
        &group2_name,
        group2_coordinates,
    );

    //group1.link(&group2);

    group1.settings.color = group1_color;
    group2.settings.color = group2_color;

    // List of groups.
    let mut groups: Vec<Group<Square>> = vec!(group1, group2);

    let children = 200;

    // Add children.
    for group in groups.iter_mut() {
        map::network::add_children(group, children);
    }

    // Where and what to call the file.
    let path= std::path::Path::new("random.png");

    let mut map = Map::new();
    map = map
        .map(&groups);

    let _ = map.image.unwrap().save(&path);
}


