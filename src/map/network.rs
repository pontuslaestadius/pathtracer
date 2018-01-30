use std::io;
use super::super::node::coordinates::*;
use super::super::node::nodelink::NodeLink;
use super::super::node::*;
use super::super::group::*;
use super::super::tools::util::*;
use super::*;

use image::Rgba;

use std::time::Instant;

pub fn create_random_network<'a>(number: u32, radius: u32) {

    // Stores all created nodes. So then they can be made in to a network.
    let mut nodes: Vec<Node> = Vec::new();
    let mut temp_nodes: Vec<Node> = Vec::new();
    let mut c: Coordinate = Coordinate::new(0, 0);

    // A list of all the names the nodes will be generated from.
    //let node_names: Vec<String> = get_node_names()?;

    for _ in 0..number {

        for node in &nodes {
            let d = gen_within_radius(&node.geo, radius);
            //let name: String = get_random_item(&node_names).clone();
            let mut this_node = Node::new(String::new(),d.clone());

            this_node.set_color(gen_rgba());

            temp_nodes.push(this_node);

            // Generates a location within a range of the previous one.
            c = gen_within_radius(&node.geo, radius); // TODO is this useless?
        }

        nodes.append(temp_nodes.as_mut());

        // Gets a name for the node.
        //let name: String = get_random_item(&node_names).clone();

        nodes.push(Node::new(String::new(),c.clone()));

        // Generates a location within a range of the previous one.
        c = gen_within_radius(&c, radius);
    }

    let connections = NodeLink::link_generic(&nodes);

    // TODO THIS IS STUPID
    super::node_and_links(&nodes, &connections);
}


pub fn create_group_network(nr_groups: u32, children_min_max: (u32, u32), radius: u32) -> Result<(), io::Error> {

    // Stores all created nodes. So then they can be made in to a network.
    let mut groups: Vec<Group> = Vec::new();

    // A list of all the names the nodes will be generated from.
    let node_names: Vec<String> = get_node_names()?;

    let zero_zero = Coordinate {x: 0, y: 0};

    // Creates the groups.
    for _ in 0..nr_groups {
        let group_coordinates = gen_within_radius(&zero_zero, radius*10);
        let group_name = get_random_item(&node_names).clone();
        groups.push(Group::new(
            group_name.as_str(),
            group_coordinates,
            gen_rgba(),
            radius
        ));
    }

    // Add the nodes to the groups.
    for mut group in groups.iter_mut() {
        add_children(&mut group, children_min_max.1);
    }

    let start = Instant::now();

    map_groups(&groups);

    let elapsed = start.elapsed();
    println!("   done - {:?}s", elapsed.as_secs());

    Ok(())
}

// Adds the number of children supplied randomly to a group.
pub fn add_children(group: &mut Group, nr_children: u32) {
    for _ in 0..nr_children {
        let coord = gen_within_radius(&group.geo, group.radius);
        let mut node = Node::new("".to_string(), coord.clone());
        node.set_color(group.gen_color(coord));
        group.push(node);
    }
}
