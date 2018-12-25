mod from;

#[cfg(test)]
mod tests {


    // Seperate from other tests, since it tests from_list for all
    // classes that have it.
    mod from_list {

        use crate::{Coordinate, Group, Node};

        // List used for calling from_list tests.
        fn get_list<'a>() -> &'a [(i16, i16); 8] {
            &[
                (0, 0),         // Default test,
                (100, 100),     // Two positive values test,
                (50, -50),      // One positive one negative test.
                (-9999, 9999),  // Larger values test.
                (0, 1),         // Close to zero test.
                (-1, -1),       // Close to zero double negative test.
                (0, 0),         // Duplicate of default test.
                (-9990, -9999), // Two large negative numbers test.
            ]
        }

        #[test]
        fn coordinate() {
            let result = Coordinate::from_list(get_list());
            assert_eq!(result.len(), 8);
        }

        #[test]
        fn node() {
            let result = Node::from_list(get_list());
            assert_eq!(result.len(), 8);
        }

        #[test]
        fn group() {
            let result = Group::from_list(get_list());
            assert_eq!(result.len(), 8);
        }

    }

    mod rotate {

        use crate::{coordinate::diff, map::network::*, Group, Location, Node};

        #[test]
        fn group() {
            let mut g = Group::new_simple(0, 0);
            add_children(&mut g, 100);
            for _ in 0..36 {
                let before = g.position();
                g.rotate(10.0);
                assert_eq!(before, g.position());
            }
        }

        fn setup_group() -> Group {
            let mut g = Group::new_simple(0, 0);
            g.radius(100);
            add_children(&mut g, 100);
            g
        }

        fn no_move(a: &Vec<Node>, b: &Vec<Node>) {
            let matching = a
                .iter()
                .zip(b.iter())
                .filter(|&(a, b)| {
                    let d = diff(a.position(), b.position());
                    // Close enough.
                    d.0 < 10 && d.1 < 10
                })
                .count();
            assert_eq!(b.len(), matching);
        }

        #[test]
        fn group_children_no_move() {
            let mut g = setup_group();
            let nodes = g.nodes().clone();
            for _ in 0..100 {
                g.rotate(0.0);
            }
            no_move(&nodes, g.nodes());
        }

        #[test]
        fn group_children_no_move_incr() {
            let mut g = setup_group();
            let nodes = g.nodes().clone();
            g.rotate(10.0);
            g.rotate(-10.0);
            no_move(&nodes, g.nodes());
        }

        #[test]
        fn group_children_inc2() {
            let mut g = setup_group();
            let nodes = g.nodes().clone();
            for _ in 0..10 {
                g.rotate(36.0);
            }
            no_move(&nodes, g.nodes());
        }

    }

    mod links {
        use crate::*;

        #[test]
        fn groups_one() {
            let mut a = Group::new_simple(0, 0);
            let b = Group::new_simple(0, 0);

            a.set().link(&b);
            assert!(a.settings.hl(0).is_ok());

            a.set().disconnect();
            assert!(a.settings.hl(0).is_err());
        }

        #[test]
        fn groups_many() {
            let mut a = Group::new_simple(0, 0);
            let mut b = Group::new_simple(0, 0);

            assert_eq!(a.settings.get_link_avail_index(), 0);
            assert!(a.settings.hl(0).is_err());

            a.settings.link(&b);
            b.set().link(&a);
            b.settings.link(&a);
            
            assert!(a.settings.hl(0).is_ok());
            assert!(b.settings.hl(0).is_ok());

            a.settings.disconnect();
            assert!(a.settings.hl(0).is_err());
            assert!(b.settings.hl(0).is_ok());
        }

        #[test]
        fn max_links() {
            let mut a = Node::new("A", Coordinate::new(0, 0));
            let nodes = Node::linked_list(Node::from_list(&[
                (0,0),
                (1,1),
                (2,2),
                (3,3),
                (4,4),
                (5,5),
                (6,6),
                (7,7),
                (8,8),
                (9,9)
            ]));

            for (i, node) in nodes.iter().enumerate() {
                a.link(node);
                if i < 5 {
                    assert!(a.hl(i).is_ok(), format!("expected {}, got {}", i, a.get_link_avail_index()));
                } else {
                    assert!(a.hl(i).is_err(), "Exceeding max_link should return Err");
                }
            }
        }

        #[test]
        fn multiple_disconnects() {
            let mut a = Node::new("A", Coordinate::new(0, 0));
            a.disconnect();
            a.disconnect();
        }

        #[test]
        fn adding_and_disconnecting() {
            let mut a = Node::new("A", Coordinate::new(0, 0));
            let mut b = Node::new("B", Coordinate::new(100, 100));
            let c = Node::new("C", Coordinate::new(200, 200));
            a.link(&b);

            assert!(a.is_directly_connected(&b));
            assert!(!b.is_directly_connected(&a));
            assert!(b.hl(0).is_err());

            b.link(&a);

            assert!(b.is_directly_connected(&a));
            assert!(a.hl(0).is_ok());
            assert!(b.hl(0).is_ok());

            assert_eq!(a.get_link_avail_index(), 1);
            assert_eq!(b.get_link_avail_index(), 1);

            a.link(&c);

            assert!(a.is_directly_connected(&c));
            assert!(a.hl(1).is_ok());
            assert_eq!(a.get_link_avail_index(), 2);

            a.disconnect();

            assert!(!a.is_directly_connected(&c));
            assert!(!a.is_directly_connected(&b));

            assert!(a.hl(0).is_err());
            assert_eq!(a.get_link_avail_index(), 0);
        }

    }

    mod path {

        use crate::{Coordinate, Node};

        #[test]
        fn nodes() {
            let mut a = Node::new("A", Coordinate::new(0, 0));
            let b = Node::new("B", Coordinate::new(100, 100));
            a.link(&b);
            assert!(a.is_directly_connected(&b));
        }

    }
}
