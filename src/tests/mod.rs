#[cfg(test)]
mod tests {

    // Seperate from other tests, since it tests from_list for all
    // classes that have it.
    mod from_list {

        use Coordinate;
        use Group;
        use Node;

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

        use Group;
        use Node;
        use coordinate::diff;
        use Location;
        use map::network::*;

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
            let matching = a.iter().zip(b.iter()).filter(|&(a, b)| {
                let d = diff(a.position(),b.position());
                // Close enough.
                d.0 < 10 && d.1 < 10
            }
            ).count();
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

    mod path {

        use Node;
        use Coordinate;

        #[test]
        fn nodes() {
            let mut a = Node::new("A", Coordinate::new(0, 0));
            let b = Node::new("B", Coordinate::new(100, 100));
            a.link(&b);
            assert!(a.is_directly_connected(&b));
        }

    }
}
