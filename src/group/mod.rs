use super::*;
use std::cmp;

/// Counts the amount of child Nodes in a list of Groups.
///
/// # Examples
/// ```
/// use pathfinder::{group, map::network, Group};
/// let mut groups = Group::from_list(&[(0, 0), (100, 100)]);
/// for group in groups.iter_mut() {
///     group.add(50);
/// }
/// assert_eq!(group::count(&groups), 100);
/// ```
pub fn count(list: &[Group]) -> usize { list.iter().fold(0, |acc, x| acc + x.nodes.len()) }

/**
Returns the the largest and smallest x and y position found in the group.

## Examples

```
# #[macro_use] extern crate pathfinder;
# use pathfinder::{group, Coordinate, Group, Node};

# fn main() {

let mut group = Group::new_simple(0, 0);
group.push(node!(100, 100));
let (min, max) = group::parameters(&group);
assert_eq!(min.x, 0);
assert_eq!(max.x, 104);

# }
```
*/
pub fn parameters(group: &Group) -> (Coordinate, Coordinate) {
    let mut min = coordinate!(0, 0);
    let mut max = coordinate!(0, 0);
    for node in &group.nodes {
        let (min2, max2) = node.min_max();
        max.x = std::cmp::max(max.x, max2.x);
        min.x = std::cmp::min(min.x, min2.x);
        max.y = std::cmp::max(max.y, max2.y);
        min.y = std::cmp::min(min.y, min2.y);
    }
    (min + group.position(), max + group.position())
}

impl<'a> PartialEq for Group {
    fn eq(&self, other: &Group) -> bool { self.hash() == other.hash() }
}

/// Adds a node to a given group, All parameters are optional except the group.
/// This is the underlying function used in Group::push(..).
pub fn add_node(group: &mut Group, name: Option<&str>, min: Option<u32>, max: Option<u32>) {
    let name = name.unwrap_or("");
    let min = min.unwrap_or(0);
    let max = max.unwrap_or_else(|| group.dynamic_radius());

    let mi = cmp::min(min, max);
    let ma = cmp::max(min, max);

    let geo = coordinate::gen_radius(group.settings.geo, mi, ma);
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

    #[test]
    fn test_add_node() {
        let mut group = cluster!();
        add_node(&mut group, None, None, None);
        add_node(&mut group, Some("name"), Some(50), Some(20));
        assert_eq!(group.nodes.len(), 2);
    }

}
