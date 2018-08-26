use super::*;
use std::cmp;

/// Counts the amount of child Nodes.
///
/// #Examples
///
/// ```
/// use pathfinder::{group, map::network, Group};
/// let mut groups = Group::from_list(&[(0, 0), (100, 100)]);
/// for group in groups.iter_mut() {
///     network::add_children(group, 50);
/// }
/// assert_eq!(group::count(&groups), 100);
/// ```
pub fn count(list: &[Group]) -> usize {
    let mut n: usize = 0;
    for g in list.iter() {
        n += g.nodes.len();
    }
    n
}

pub fn get_parameters(group: &Group) -> (Coordinate, Coordinate) {
    let mut min_x: i16 = 0;
    let mut min_y: i16 = 0;
    let mut max_x: i16 = 0;
    let mut max_y: i16 = 0;

    for node in &group.nodes {
        let (min, max) = node.get_parameters();
        max_x = std::cmp::max(max_x, max.x);
        min_x = std::cmp::min(min_x, min.x);
        max_y = std::cmp::max(max_y, max.y);
        min_y = std::cmp::min(min_y, min.y);
    }
    (Coordinate::new(min_x, min_y), Coordinate::new(max_x, max_y))
}

/// Adds a node to a given group, All parameters are optional except the group.
pub fn add_node(
    group: &mut Group,
    mut name: Option<&str>,
    mut min: Option<u32>,
    mut max: Option<u32>,
) {
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
    use super::{super::Node, *};

    #[test]
    fn test_count_none() {
        let groups = Group::from_list(&[(0, 0), (100, 100)]);
        assert_eq!(count(&groups), 0);
    }

    #[test]
    fn test_count_some() {
        let mut groups = Group::from_list(&[(0, 0), (100, 100)]);
        groups[0].nodes = Node::from_list(&[(0, 0), (0, 0)]);
        assert_eq!(count(&groups), 2);
    }

}
