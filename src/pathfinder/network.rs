/*
/*
    Network
    -------
    Binds the nodes and the connections via an extra layer of abstraction
*/

pub struct Network<'a>  {
    wrappers: Vec<Wrapper<'a>>
}

impl<'a> Network<'a> {

    fn new(nodes: [Node], links: [NodeLink]) -> Network {
        let wrappers = Vec::new();

        // TODO implement

        Network (
            wrappers
        )
    }

}
*/

use std::io;
use pathfinder::node::coordinates::Coordinates;
use pathfinder::node::nodelink::NodeLink;
use pathfinder::node::*;
use super::tools::util::*;
use super::map;


pub fn create_random_network(number: u32, radius: i16) -> Result<(), io::Error> {

    debug_print("creating node network..");

    // Stores all created nodes. So then they can be made in to a network.
    let mut nodes: Vec<Node> = Vec::new();
    let mut temp_nodes: Vec<Node> = Vec::new();
    let mut connections: Vec<NodeLink> = Vec::new();
    let mut c: Coordinates = Coordinates::new(0,0);

    // A list of all the names the nodes will be generated from.
    let node_names: Vec<String> = get_node_names()?;

    debug_print("   creating nodes..");
    // For the number of nodes in the network.
    for _ in 0..number {

        for node in &nodes {
            let d = Coordinates::gen_within_radius(node.geo.clone(), radius);
            let name: String = get_random_item(&node_names).clone();
            let this_node = Node::new(name,d.clone());

            temp_nodes.push(this_node);

            // Generates a location within a range of the previous one.
            c = Coordinates::gen_within_radius(node.geo.clone(), radius); // TODO is this useless?
        }

        nodes.append(temp_nodes.as_mut());

        // Gets a name for the node.
        let name: String = get_random_item(&node_names).clone();

        nodes.push(Node::new(name,c.clone()));

        // Generates a location within a range of the previous one.
        c = Coordinates::gen_within_radius(c.clone(), radius);
    }
    debug_print("   done");

    debug_print("   linking nodes..");
    connections = NodeLink::link(&nodes);

    debug_print("   done");

    /*
    debug_print("   saving NodeLink(s)..");
    for con in connections.iter() {
        con.save();
    }
    debug_print("   done");
    */

    debug_print("   generating map..");
    // map::map_node(&nodes); // This feature works so it is commented out to try new things.
    map::map_node_and_links(&nodes, &connections);
    debug_print("   done");

    /*
    debug_print("   saving Node(s)..");
    let _ = Node::save_list(&nodes);
    debug_print("   done");
    */

    debug_print("done");
    Ok(())
}