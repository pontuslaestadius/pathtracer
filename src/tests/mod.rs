#[cfg(test)]
mod tests {

    // ------------------------------------------------------------
    // Each nested module represents one file that is being tested.
    // ------------------------------------------------------------

    // Seperate from other tests, since it tests from_list for all 
    // classes that have it.
    mod from_list {

        use Coordinate;
        use Node;
        use Group;

        // List used for calling from_list tests.
        fn get_list<'a>() -> &'a [(i16,i16); 8] {
        &[
            (0, 0), // Default test,
            (100, 100), // Two positive values test,
            (50, -50), // One positive one negative test.
            (-9999, 9999), // Larger values test.
            (0, 1), // Close to zero test.
            (-1, -1), // Close to zero double negative test.
            (0, 0), // Duplicate of default test.
            (-9990, -9999) // Two large negative numbers test.
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

    mod coordinates {
        use Coordinate;
        use node::coordinates::{gen_radius, gen_within_radius};

        // Tests coordinates equality.
        #[test]
        fn test_eq() {

            // Declaring
            let co1: Coordinate = Coordinate::new(1, 1);
            let co2: Coordinate = co1.clone();
            let co3: Coordinate = Coordinate::new(2, 2);

            // Asserts
            assert_eq!(co1 == co2, true);
            assert_ne!(co1 == co3, true);
            assert_eq!(co1 < co3, true);
        }

        // Tests simple coordinate positioning. And random placements.
        // NOT PURE USES RAND
        #[test]
        fn test_gen_within_radius() {

            // Default
            let co1: Coordinate = Coordinate::new(1, 1);
            // Get min and max points. Which the radius can't exceed.
            let co4: Coordinate = Coordinate::new(102, 102);
            let co5: Coordinate = Coordinate::new(-102, -102);

            // Since randomness is applied. It's effect is lowered by using many iterations.
            for _ in 0..100 {
                let co6: Coordinate = gen_within_radius(co1, 100);
                assert_eq!(co4 > co6, true);
                assert_eq!(co5 < co6, true);
            }

        }


        // Tests random coordinate positioning.
        // NOT PURE USES RAND
        #[test]
        fn test_gen_radius() {

            // Default
            let co1: Coordinate = Coordinate::new(1, 1);
            // Get min and max points. Which the radius can't exceed.
            let co4: Coordinate = Coordinate::new(102, 102);
            let co5: Coordinate = Coordinate::new(-102, -102);

            // Since randomness is applied. It's effect is lowered by using many iterations.
            for _ in 0..100 {
                let co6: Coordinate = gen_radius(co1, 0, 100);
                assert_eq!(co4 > co6, true);
                assert_eq!(co5 < co6, true);
            }
        }


        // Tests getting the difference of the coordinates.
        #[test]
        fn test_diff() {
            // Declare
            let co1: Coordinate = Coordinate::new(1, 1);
            let co2: Coordinate = Coordinate::new(102, 102);
            let co3: Coordinate = Coordinate::new(-102, -102);

            // Assert
            assert_eq!(co1.diff(co2) == (101, 101), true);
            assert_eq!(co1.diff(co3) == (103, 103), true);
            assert_eq!(co2.diff(co3) == (204, 204), true);
            assert_eq!(co1.diff(co1) == (0, 0), true);

        }

        // Tests cloning.
        #[test]
        fn test_clone() {
            let co1: Coordinate = Coordinate::new(1, 1);
            let co2: Coordinate = Coordinate::new(9999, 9999);
            assert_eq!(co1 == co1.clone(), true);
            assert_eq!(co2 == co2.clone(), true);
        }
    }

}
