/*!
  Connects and paths between different connected Nodes.
*/

use super::*;
use std::io::{self, Error, ErrorKind};

/**
Weighted Node
*/
struct WNodes {
    nodes: Vec<Node>,
    weight: u32,
}

/**
 Paths between two different points that are connected.


## Errors

 The provided A and B don't exist in the network.

 The path could not be found.
 */
pub fn path<'a>(
    network: &'a Network<Node>,
    a: &str,
    b: &str,
    algorithm: &Fn(&Network<Node>, Node, Node) -> io::Result<Vec<Node>>,
) -> io::Result<Vec<Node>> {
    let goal = network.get(b).expect("goal does not exist in network");
    let start = network.get(a).expect("start does not exist in network");
    algorithm(&network, start, goal)
}

/**
Retrieves a node from a network.
*/
pub fn get(network: &Network<Node>, element: &str) -> Option<Node> {
    let tmp = node!(element, 0, 0);
    for (i, elem) in network.hash_map.iter().enumerate() {
        if elem.is_some() && i == tmp.hash as usize % consts::NETWORK_REM {
            return network.hash_map[i];
        }
    }
    None
}

/**
 Creates a path using the 'shortest leg' in the journey at each stop.

 The shorest leg means that for every occurence of a path, the alternatives are sorted and the shortest is always selected.


## Errors

 The path could not be found.


 ## Panics

 The provided A and B don't exist in the network.
 */
pub fn path_shortest_leg<'a>(
    network: &'a Network<Node>,
    start: Node,
    goal: Node,
) -> io::Result<Vec<Node>> {
    // Create a new Branch-off path.
    let format = |mut nodes: Vec<Node>, link: &HL, acc: u32| -> WNodes {
        let node = network.hash_map[link.t as usize % consts::NETWORK_REM].unwrap();
        let weight = acc + coordinate::distance(nodes.first().unwrap().geo, node.geo);
        nodes.insert(0, node);
        WNodes { weight, nodes }
    };

    // Create the queue from connected links.
    let mut queue: Vec<WNodes> = start
        .links()
        .iter()
        .filter(|x| x.is_connected())
        .map(|x| format(vec![start], x, 0))
        .collect::<Vec<_>>();

    while !queue.is_empty() {
        // Sort the queue based on weight.
        queue.sort_by_key(|wn| wn.weight);

        // Pull the closest path from the Queue.
        let wnodes = queue.remove(0);

        // Get the current Node in the closest path.
        let current = wnodes.nodes.first().unwrap();

        // End if we are at the goal.
        if current.hash == goal.hash {
            return Ok(wnodes.nodes);
        }

        // Push new paths to the queue.
        let _ = current
            .links()
            .iter()
            .filter(|x| x.is_connected())
            .map(|x| queue.push(format(wnodes.nodes.clone(), x, wnodes.weight)))
            .collect::<Vec<_>>();
    }

    // If we run out of items in the Queue, and we have not reacted
    // the goal, the path is invalid. And does not exist.
    Err(Error::new(ErrorKind::Other, "not a valid path"))
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn simple_network_shortest_leg() {
        let nodes = Node::from_list(&[(0, 0), (10, 10), (20, 20), (30, 30)]);
        let nodes = Node::linked_list(nodes);
        let net = Network::new(nodes);
        let path = path(&net, "D", "A", &path_shortest_leg).unwrap();
        assert_eq!(path.len(), 4);
    }

    #[test]
    fn simple_network() {
        let nodes = Node::from_list(&[(0, 0), (10, 10), (20, 20), (30, 30)]);
        let nodes = Node::linked_list(nodes);
        let path = Network::new(nodes).path("A", "D").unwrap();
        assert_eq!(path.len(), 4);
    }

    #[test]
    fn simple_networks_have_same_path() {
        let nodes = Node::from_list(&[(0, 0), (10, 10), (20, 20), (30, 30)]);
        let nodes = Node::linked_list(nodes);

        let net = Network::new(nodes);
        let mut path_sl = path(&net, "D", "A", &path_shortest_leg).unwrap();
        let path = net.path("A", "D").unwrap();
        path_sl.reverse();

        let f = |&p: &Node| p.geo;
        let v1 = path.iter().map(f).collect::<Vec<_>>();
        let v2 = path_sl.iter().map(f).collect::<Vec<_>>();

        assert_eq!(v1, v2);
    }

}
