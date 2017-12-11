#[cfg(test)]
mod tests {





    // Holds all tests for the module coordinates
    mod coordinates {
        use node::coordinates::*;

        fn coordinates() {
            let co1: Coordinates = Coordinates::new(1, 1);
            let co2: Coordinates = co1.clone();
            let co3: Coordinates = Coordinates::new(2, 2);

            // Comparing Coordinates
            let res1: bool = co1 == co2;
            let res2: bool = co1 == co3;
            let res3: bool = co1 < co3;

            assert_eq!(res1, true);
            assert_ne!(res2, true);
            assert_eq!(res3, true);

            let co4: Coordinates = Coordinates::new(102, 102);

            // Since randomness is applied. It's effect is lowered by using many iterations.
            for _ in 0..100 {
                let co5: Coordinates = Coordinates::gen_within_radius(co1.clone(), 100);
                assert_eq!(co4 > co5, true)
            }

        }

        
    }



}