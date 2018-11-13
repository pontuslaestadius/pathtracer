use super::{super::coordinate::*, *};

pub fn path<'a>(
    network: &'a Network<Node>,
    a: &str,
    b: &str,
    algorithm: &Fn(&Network<Node>, &str, &str) -> Vec<Node>,
) -> Vec<Node> {
    let _goal = network.get(b).expect("goal does not exist in network");
    let start = network.get(a).expect("start does not exist in network");

    if start.links().is_empty() {
        return Vec::new();
    }

    algorithm(&network, a, b)
}

/// Retrieves a node from a network.
pub fn get(network: &Network<Node>, element: &str) -> Option<Node> {
    let tmp = Node::new(element, Coordinate::new(0, 0));
    for (i, elem) in network.hash_map.iter().enumerate() {
        if elem.is_some() && i == (tmp.hash % 666) as usize {
            return network.hash_map[i];
        }
    }
    None
}

pub fn path_shortest_leg<'a>(network: &'a Network<Node>, a: &str, b: &str) -> Vec<Node> {
    let goal = network.get(b).expect("goal does not exist in network");
    let first = network.get(a).expect("start does not exist in network");
    let mut queue: Vec<(u32, Vec<Node>)> = Vec::new();

    let format = |mut from: Vec<Node>, link: &HL, acc: u32| -> (u32, Vec<Node>) {
        let node_opt = network.hash_map[(link.t % 666) as usize];
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

    for link in first.links().iter() {
        if link.t != 0 {
            queue.push(format(vec![first], link, 0));
        }
    }

    while !queue.is_empty() {
        queue.sort_by_key(|k| k.0);
        let (dis, path) = queue.remove(0);
        let current = path[0];

        for link in current.links().iter() {
            if link.t != 0 {
                queue.push(format(path.clone(), link, dis));
            }
        }

        if current.hash == goal.hash {
            return path;
        }
    }
    Vec::new()
}

/// Adds the number of children supplied, positioned randomly to a group.
///
/// # Examples
///
/// ```
/// use pathfinder::{group, map::network, Group};
///
/// let mut groups = Group::from_list(&[(0, 0), (100, 100)]);
/// for group in groups.iter_mut() {
///     network::add_children(group, 50);
/// }
/// assert_eq!(group::count(&groups), 100);
/// ```
pub fn add_children(group: &mut Group, nr_children: u32) {
    for _ in 0..nr_children {
        let co = gen_within_radius(group.settings.geo, group.settings.size());
        let mut node = Node::new("", co);
        node.color = group.gen_color(co);
        group.push(node);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn children_none() {
        let mut group = Group::new_simple(0, 0);
        add_children(&mut group, 0);
        assert_eq!(group.nodes.len(), 0);
    }

    #[test]
    fn children_some() {
        let mut group = Group::new_simple(0, 0);
        add_children(&mut group, 50);
        assert_eq!(group.nodes.len(), 50);
    }

    #[test]
    fn children_many() {
        let mut group = Group::new_simple(0, 0);
        add_children(&mut group, 9999);
        assert_eq!(group.nodes.len(), 9999);
    }
}
