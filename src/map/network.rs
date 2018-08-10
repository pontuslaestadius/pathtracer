use super::*;
use super::super::node::coordinates::*;

/// Adds the number of children supplied, positioned randomly to a group.
///
/// #Examples
///
/// ```
/// use pathfinder::Group;
/// use pathfinder::map::network;
/// use pathfinder::group;
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
        let mut group = Group::new_simple(0,0);
        add_children(&mut group, 0);
        assert_eq!(group.nodes.len(), 0);
    }

    #[test]
    fn children_some() {
        let mut group = Group::new_simple(0,0);
        add_children(&mut group, 50);
        assert_eq!(group.nodes.len(), 50);
    }

    #[test]
    fn children_many() {
        let mut group = Group::new_simple(0,0);
        add_children(&mut group, 9999);
        assert_eq!(group.nodes.len(), 9999);
    }
}

