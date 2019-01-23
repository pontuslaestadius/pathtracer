use super::*;
use std::io::{self, Error, ErrorKind};

/// Paths between two different points that are connected.
/// Will return an Error if the provided A and B don't exist in the network,
/// or if the path could not be found.
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

/// Retrieves a node from a network.
pub fn get(network: &Network<Node>, element: &str) -> Option<Node> {
    let tmp = Node::new(element, Coordinate::new(0, 0));
    for (i, elem) in network.hash_map.iter().enumerate() {
        if elem.is_some() && i == tmp.hash as usize % consts::NETWORK_REM {
            return network.hash_map[i];
        }
    }
    None
}

/// Creates a path using the 'shortest leg' in the journey at each stop.
pub fn path_shortest_leg<'a>(
    network: &'a Network<Node>,
    start: Node,
    goal: Node,
) -> io::Result<Vec<Node>> {
    let format = |mut from: Vec<Node>, link: &HL, acc: u32| -> (u32, Vec<Node>) {
        let node_opt = network.hash_map[link.t as usize % consts::NETWORK_REM];
        let node = node_opt.unwrap_or_else(|| {
            panic!(
                "Node missing in network. From: {:?}, Link:
        {:?}",
                from, link
            )
        });
        let dis = coordinate::distance(from[0].geo, node.geo);
        from.insert(0, node);
        (acc + dis, from)
    };

    let mut queue: Vec<(u32, Vec<Node>)> =
        start
            .links()
            .iter()
            .filter(|x| x.is_connected())
            .fold(vec![], |mut acc, x| {
                acc.push(format(vec![start], x, 0));
                acc
            });

    while !queue.is_empty() {
        queue.sort_by_key(|k| k.0);
        let (dis, path) = queue.remove(0);
        let current = path[0];

        if current.hash == goal.hash {
            return Ok(path);
        }

        for link in current.links().iter() {
            if link.is_connected() {
                queue.push(format(path.clone(), link, dis));
            }
        }
    }
    Err(Error::new(ErrorKind::Other, "not a valid path"))
}

#[cfg(test)]
mod tests {}
