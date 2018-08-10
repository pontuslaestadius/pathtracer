use super::Group;

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
