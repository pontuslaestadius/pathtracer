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
use pathfinder::{node, map, data, group};

fn main() {

    pathfinder::map::network::create_random_network(5, 50);

    /*
    let _ = pathfinder::pathfinder::network::create_group_network(5000, (100, 1000), 20);
    */

    /*

    // Ideal implementation example:

    // Coordinates for the groups.
    let group1_coordinates = node::coordinates::Coordinates::new(0,0);
    let group2_coordinates = node::coordinates::Coordinates::new(250,150);

    // Names for the groups.
    let group1_name = "john doe".to_string();
    let group2_name = "jane doe".to_string();

    // Colors for the groups.
    let group1_color = Rgba {data: [250, 20, 20, 255]};
    let group2_color = Rgba {data: [20, 20, 250, 255]};

    // Maximum distance for child nodes.
    let child_node_spawn_radius = 25;

    // Group 1.
    let group1 = Group::new(
        group1_name,
        group1_coordinates,
        group1_color,
        child_node_spawn_radius
    );

    // Group 2.
    let group2 = Group::new(
        group2_name,
        group2_coordinates,
        group2_color,
        child_node_spawn_radius
    );

    // List of groups.
    let mut groups = vec!(group1, group2);

    let children = 20000;

    // Add children.
    for group in groups.iter_mut() {
        network::add_children(group, children);
    }

    // Create a link between the groups.
    let links = vec!(node::link::Link::new(&groups.get(0).unwrap().geo, &groups.get(1).unwrap().geo));

    // Where and what to call the file.
    let save_path = "examples/example3.png";

    // Maps them on an image and draw it.
    map::groups_and_links(&groups, &links, save_path);

    */

}


