use std::io;
use pathfinder::node::coordinates::*;
use pathfinder::node::nodelink::NodeLink;
use pathfinder::node::*;
use pathfinder::group::*;
use super::tools::util::*;
use super::map;

use image::Rgba;

use std::time::Instant;


pub fn create_random_network<'a>(number: u32, radius: u32) -> Result<([Node], [NodeLink<'a>]), io::Error> {

    // Stores all created nodes. So then they can be made in to a network.
    let mut nodes: Vec<Node> = Vec::new();
    let mut temp_nodes: Vec<Node> = Vec::new();
    let mut c: Coordinates = Coordinates::new(0,0);

    // A list of all the names the nodes will be generated from.
    let node_names: Vec<String> = get_node_names()?;

    for _ in 0..number {

        for node in &nodes {
            let d = gen_within_radius(&node.geo, radius);
            let name: String = get_random_item(&node_names).clone();
            let mut this_node = Node::new(name,d.clone());

            this_node.set_color(gen_rgba());

            temp_nodes.push(this_node);

            // Generates a location within a range of the previous one.
            c = gen_within_radius(&node.geo, radius); // TODO is this useless?
        }

        nodes.append(temp_nodes.as_mut());

        // Gets a name for the node.
        let name: String = get_random_item(&node_names).clone();

        nodes.push(Node::new(name,c.clone()));

        // Generates a location within a range of the previous one.
        c = gen_within_radius(&c, radius);
    }

    debug_print("   linking nodes..");
    let connections = NodeLink::link(&nodes);

    /*
    debug_print("   saving NodeLink(s)..");
    for con in connections.iter() {
        con.save();
    }
    */
    Ok((nodes, connections))

    /*
    debug_print("   saving Node(s)..");
    let _ = Node::save_list(&nodes);
    */

    Ok(())
}

pub fn create_group_network(nr_groups: u32, children_min_max: (u32, u32), radius: u32) -> Result<(), io::Error> {
    debug_print("creating group network..");

    // Stores all created nodes. So then they can be made in to a network.
    let mut groups: Vec<Group> = Vec::new();

    // A list of all the names the nodes will be generated from.
    let node_names: Vec<String> = get_node_names()?;

    let zero_zero = Coordinates {x: 0, y: 0};

    // Creates the groups.
    for _ in 0..nr_groups {
        let group_coordinates = gen_within_radius(&zero_zero, radius*10);
        let group_name = get_random_item(&node_names).clone();
        groups.push(Group::new(
            group_name,
            group_coordinates,
            gen_rgba(),
            radius
        ));
    }

    // Add the nodes to the groups.
    for mut group in groups.iter_mut() {
        add_children(&mut group, children_min_max.1);
    }

    debug_print("   generating map..");
    let start = Instant::now();

    map::map_groups(&groups);

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
