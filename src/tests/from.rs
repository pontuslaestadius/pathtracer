#[cfg(test)]
mod from {
    use crate::*;

    #[test]
    fn coordinate_conversion() {
        let node = Node::new("", Coordinate::new(6, 1));
        let c = Coordinate::from(node);
        assert_eq!(c, Coordinate::new(6, 1));

        let group = Group::new_simple(4, 3);
        let c = Coordinate::from(group);
        assert_eq!(c, Coordinate::new(4, 3));
    }

    #[test]
    fn node_conversion() {
        let c = Coordinate::new(6, 1);
        let node = Node::from(c);
        assert_eq!(node.position(), Coordinate::new(6, 1));

        let group = Group::new_simple(4, 3);
        let node = Node::from(group);
        assert_eq!(node.position(), Coordinate::new(4, 3));
    }

    #[test]
    fn group_conversion() {
        let c = Coordinate::new(6, 1);
        let group = Group::from(c);
        assert_eq!(group.position(), Coordinate::new(6, 1));

        let node = Node::new("", Coordinate::new(4, 3));
        let group = Group::from(node);
        assert_eq!(group.position(), Coordinate::new(4, 3));
    }
}
