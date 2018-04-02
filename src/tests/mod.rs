#[cfg(test)]
mod tests {

    // ------------------------------------------------------------
    // Each nested module represents one file that is being tested.
    // ------------------------------------------------------------

    mod coordinates {
        use node::coordinates::*;

        // Tests coordinates equality.
        #[test]
        fn test_eq() {

            // Declaring
            let co1: Coordinate = Coordinate::new(1, 1);
            let co2: Coordinate = co1.clone();
            let co3: Coordinate = Coordinate::new(2, 2);
            let co4: Coordinate = Coordinate::new(300, 2);

            // Comparing
            let res1: bool = co1 == co2;
            let res2: bool = co1 == co3;
            let res3: bool = co1 < co3;

            // Asserts
            assert_eq!(res1, true);
            assert_ne!(res2, true);
            assert_eq!(res3, true);
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
                let co6: Coordinate = gen_within_radius(&co1, 100);
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
                let co6: Coordinate = gen_radius(&co1, 0, 100);
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
            assert_eq!(co1.diff(&co2) == (101, 101), true);
            assert_eq!(co1.diff(&co3) == (103, 103), true);
            assert_eq!(co2.diff(&co3) == (204, 204), true);
            assert_eq!(co1.diff(&co1) == (0, 0), true);

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

    mod util {
        use tools::util::*;

        #[test]
        fn test_border() {
            assert_eq!(border(0, 0), 0);
            assert_eq!(border(0, -55), 0);
            assert_eq!(border(0, -255), 0);
            assert_eq!(border(0, 55), 55);
            assert_eq!(border(0, 255), 255);

            assert_eq!(border(255, 0), 255);
            assert_eq!(border(255, -255), 0);
            assert_eq!(border(255, -255), 0);
            assert_eq!(border(100, 100), 200);
        }

    }


    mod link {

    }

}