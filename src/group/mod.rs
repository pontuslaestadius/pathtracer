/*!
Extensively parameterized group features.

These can be interacted with via group function implementations.

Here they also can be used for pre precise control.
 */

use super::*;
use std::cmp;

/**
Counts the amount of child Nodes in a list of Groups.


## Examples

```
# use pathtracer::*;
let mut groups = Group::from_list(&[(0, 0), (100, 100)]);
for group in groups.iter_mut() {
    group.add(50);
}
assert_eq!(group::count(&groups), 100);
```
*/
pub fn count(list: &[Group]) -> usize {
    list.iter().fold(0, |acc, x| acc + x.nodes.len())
}

impl<'a> PartialEq for Group {
    fn eq(&self, other: &Group) -> bool {
        self.hash() == other.hash()
    }
}

/**
Adds a node to a given group, All parameters are optional except the group.

This is the underlying function used in Group::push(..).


## Examples

```
# use pathtracer::*;
# let mut group = Group::new("", Coordinate::new(0, 0));
group::add_node(&mut group, Some("Name"), Some(5), Some(9));
assert_eq!(group.nodes.len(), 1);
assert!(group.nodes[0].position().lt(10));
```
*/
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
