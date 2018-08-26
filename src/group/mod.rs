use super::{Node, Group};
use super::coordinate;

use std::cmp;

/// Counts the amount of child Nodes.
///
/// #Examples
///
/// ```
/// use pathfinder::group;
/// use pathfinder::map::network;
/// use pathfinder::Group;
/// let mut groups = Group::from_list(&[(0, 0), (100, 100)]);
/// for group in groups.iter_mut() {
///     network::add_children(group, 50);
/// }
/// assert_eq!(group::count(&groups), 100);
/// ```
pub fn count(list: &[Group]) -> usize {
    let mut n: usize = 0;
    for g in list.iter() {
        n+=g.nodes.len();
    }
    n
}

/// Adds a node to a given group, All parameters are optional except the group.
pub fn add_node(group: &mut Group, mut name: Option<&str>, mut min: Option<u32>, mut max: Option<u32>) {
    if name.is_none() {
        name = Some("");
    }

    if min.is_none() {
        min = Some(0);
    }

    if max.is_none() {
        max = Some(group.get_dynamic_radius());
    }
    let min = min.unwrap();
    let max = max.unwrap();
    let name = name.unwrap();

    let tmp = min;
    let min = cmp::min(min, max);
    let max = cmp::max(tmp, max);

    let geo = coordinate::gen_radius(group.settings.geo, min, max);
    let mut node = Node::new(name, geo);
    node.color = group.gen_color(geo);
    node.radius = group.settings.radius;
    group.push(node);
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::Node;

    #[test]
    fn test_count_none() {
        let groups = Group::from_list(&[(0, 0), (100, 100)]);
        assert_eq!(count(&groups), 0);
    }

    #[test]
    fn test_count_some() {
        let mut groups = Group::from_list(&[(0, 0), (100, 100)]);
        groups[0].nodes = Node::from_list(&[(0, 0),(0, 0)]);
        assert_eq!(count(&groups), 2);
    }

}
