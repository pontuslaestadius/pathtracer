use super::{super::coordinate::*, *};

pub fn path<'a>(
    network: &'a Network<Node>,
    a: &str,
    b: &str,
    algorithm: &Fn(&Network<Node>, &str, &str) -> Vec<Node>,
) -> Vec<Node> {
    let _goal = network.get(b).expect("goal does not exist in network");
    let start = network.get(a).expect("start does not exist in network");

    if start.get_links().is_empty() {
        return Vec::new();
    }

    algorithm(&network, a, b)
}

pub fn get(network: &Network<Node>, element: &str) -> Option<Node> {
    let tmp = Node::new(element, Coordinate::new(0, 0));
    for (i, elem) in network.hash_map.iter().enumerate() {
        if elem.is_none() {
            continue;
        }
        if i == (tmp.hash % 666) as usize {
            return network.hash_map[i];
        }
    }
    None
}

pub fn path_shortest_leg<'a>(network: &'a Network<Node>, a: &str, b: &str) -> Vec<Node> {
    let _goal = network.get(b).expect("goal does not exist in network");
    let first = network.get(a).expect("start does not exist in network");

    let mut weighted_path: Vec<(u32, Vec<Node>)> = Vec::new();
    for l in first.get_links().iter() {
        let node_opt = network.hash_map[(l.to_hash % 666) as usize];
        if node_opt.is_none() {
            continue;
        }
        let node = node_opt.unwrap();
        let dis = coordinate::distance(first.geo, node.geo);
        weighted_path.push((dis, vec![first, node]));
    }

    if weighted_path.is_empty() {
        panic!("No more paths!");
    }

    let (_dis, path) = weighted_path.remove(0);
    path

    /*
     * For each link in starting node.
     * Make a weighted list of sum_distance for each available path.
     * Pick the lowest weighted path.
     * Once the path is at the goal, we stop.
     * Generate path from numbers.
     */
}

/// Adds the number of children supplied, positioned randomly to a group.
///
/// #Examples
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
        let co = gen_within_radius(group.settings.geo, group.settings.get_size());
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
