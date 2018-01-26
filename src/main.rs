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

use rand::*;
use pathfinder::{node, map, data, group};
use pathfinder::node::*;

fn main() {


    // The tag to find to group them by.
    let find: String = "Author".to_string();

    let nothing = Vec::new();

    // Save the tag to use to convert the data.
    let tag = data::Tag {collection: find, ignore: nothing};

    // Fetches the log, replace it with your log directory.
    let log = "resources/log.txt";

    // Use the log directory and the tag to create the groups.
    let (groups, links) = data::convert_file(log, &tag);

    // Count the groups and nodes.
    let (g, n) = group::count(&groups);

    // Print them.
    println!("{:?} groups with {} nodes", g, n);

    // Save path for the final result.
    let save_path = "visual_data.png";

    // Map them to an RGBA Image and saves it.
    map::groups_and_links(&groups, &links, save_path);

    /*

    //pathfinder::map::network::create_random_network(2, 70);

    let mut nodes: Vec<Node> = Vec::new();
    let mut links = Vec::new();

    for i in 0..33   {
        let dif = 10;
        let start = coordinates::Coordinates::new(-(dif/2)*i,-(dif/2)*i);
        let mut new_nodes = figure::rectangle(&start, dif*i, dif*i);
        nodes.append(&mut new_nodes)
    }


    let start = coordinates::Coordinate::new(0,0);
    nodes = figure::rectangle(&start, 50, 50);


    for (i, item) in nodes.iter().enumerate() {
        for (j, item2) in nodes.iter().enumerate() {
            if i == j {continue};

            links.push(
                link::Link::new(&item.geo, &item2.geo)
            );

        }
    }

    // Shuffle links.
    let mut rng = thread_rng();
    //rng.shuffle(&mut links);



    pathfinder::map::node_and_links(&nodes, &links);


    let _ = pathfinder::pathfinder::network::create_group_network(5000, (100, 1000), 20);




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


