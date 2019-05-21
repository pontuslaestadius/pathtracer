/*!
Extra functionality for coordinates.
 */

extern crate pythagoras;
extern crate rand;

use super::{tools::roll, Coordinate};
use std::{cmp::Ordering, f64};

/**
Constructs a vector of generic structs from a given list convered to Coordinates.

## See also

examples/city.rs

*/
pub fn from_list<T>(list: &[(i16, i16)], get: &Fn(Coordinate, usize) -> T) -> Vec<T> {
    let mut result: Vec<T> = Vec::new();
    for (i, &(x, y)) in list.iter().enumerate() {
        result.push(get(Coordinate::new(x, y), i));
    }
    result
}

/**
Constructs a randomly positioned coordinate.
*/
pub fn gen() -> Coordinate {
    Coordinate {
        x: rand::random::<i16>(),
        y: rand::random::<i16>(),
    }
}

/**
Calculate a coordinate based on another coordinate and a function.


## Examples

```
# use pathfinder::{coordinate::*, Coordinate};
let c1 = Coordinate::new(0, 0);
let f = |i: usize| -> Coordinate { Coordinate::new(i as i16, i as i16) };
let c2 = calc(c1, 5, &f);
assert_eq!(c2, Coordinate { x: 5, y: 5 });
```

## See also

examples/node_plot.rs

*/
pub fn calc(start: Coordinate, variable: usize, call: &Fn(usize) -> Coordinate) -> Coordinate {
    start + call(variable)
}

/**
get difference in distance.


## Examples

```
# #[macro_use] use pathfinder::*;
# use pathfinder::coordinate::*;
# fn main() {
let difference = diff(coordinate!(), coordinate!(100));
assert_eq!(difference, (100, 100));
# }
```
*/
pub fn diff(c1: Coordinate, c2: Coordinate) -> (i16, i16) {
    let c = (c1 - c2).abs();
    (c.x, c.y)
}

/**
 Get the distance between two Coordinates'.


## Examples

```
# #[macro_use] use pathfinder::*;
# use pathfinder::coordinate::*;
# fn main() {
let distance = distance(coordinate!(), coordinate!(3, 4));
assert_eq!(distance, 5);
# }
```
*/
pub fn distance(a: Coordinate, b: Coordinate) -> u32 {
    let diff = diff(a, b);
    pythagoras::theorem(diff.0, diff.1) as u32
}

/**
 Generate a Coordinate from a given Coordinate and randomly places it within a radius.


## Examples

```
# #[macro_use] use pathfinder::*;
# use pathfinder::coordinate::*;
# fn main() {
let c2 = gen_within_radius(coordinate!(), 100);
# }
```
*/
pub fn gen_within_radius(coord: Coordinate, radius: u32) -> Coordinate {
    gen_radius(coord, 0, radius)
}

/**
Generate a Coordinate from a given Coordinate and randomly places it within
a min and max radius.


## Examples

```
# use pathfinder::{coordinate, Coordinate};
let c1 = Coordinate::new(0, 0);
let c2 = coordinate::gen_radius(c1, 50, 100);
```
*/
pub fn gen_radius(coord: Coordinate, min: u32, max: u32) -> Coordinate {
    // Randomly gets the radius of the circle.
    let r = f64::from(roll(min, max));

    // gets a point on the circle's circumference.
    let circle = |a: f64, b: f64| a + r * b;

    // Gets a random angle.
    let angle = roll(0u32, 3600u32);
    let a: f64 = f64::consts::PI * 0.001 * f64::from(angle);

    let x = circle(f64::from(coord.x), a.cos()) as i16;
    let y = circle(f64::from(coord.y), a.sin()) as i16;

    Coordinate { x, y }
}

/**
Rotates the provide Vec around the axis in place.

If deg == 0.0, no rotation occurs.


## Examples

```
# #[macro_use] extern crate pathfinder;
# use pathfinder::{coordinate::*, Coordinate, Node};
# fn main() {
let mut v = vec![node!(0, 100)];
rotate_around_axis(coordinate!(), &mut v, 90.0);
assert_eq!(v.remove(0).geo, Coordinate::new(100, 0));
# }
```


## See also

examples/node_plot.rs

*/
pub fn rotate_around_axis(axis: Coordinate, points: &mut Vec<super::Node>, deg: f64) {
    if deg == 0.0 {
        return;
    }

    for p in points.iter_mut() {
        let radius = f64::from(distance(axis, p.geo));
        let diff = p.geo - axis;
        let base = f64::from(diff.x).atan2(f64::from(diff.y));
        let angle = base + (deg * f64::consts::PI / 180.0);

        p.geo.y = axis.y + f64::round(angle.cos() * radius) as i16;
        p.geo.x = axis.x + f64::round(angle.sin() * radius) as i16;
    }
}

impl Ord for Coordinate {
    fn cmp(&self, other: &Coordinate) -> Ordering {
        (self.x + self.y).cmp(&(other.x + other.y))
    }
}

impl PartialOrd for Coordinate {
    fn partial_cmp(&self, other: &Coordinate) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Coordinate {
    fn eq(&self, other: &Coordinate) -> bool {
        (self.x == other.x) && (self.y == other.y)
    }
}

impl std::ops::Add for Coordinate {
    type Output = Coordinate;

    fn add(self, other: Coordinate) -> Coordinate {
        Coordinate::new(self.x + other.x, self.y + other.y)
    }
}

impl std::ops::Sub for Coordinate {
    type Output = Self;

    fn sub(self, other: Coordinate) -> Coordinate {
        Coordinate::new(self.x - other.x, self.y - other.y)
    }
}

impl std::ops::AddAssign for Coordinate {
    fn add_assign(&mut self, other: Coordinate) {
        self.x += other.x;
        self.y += other.y;
    }
}

impl std::ops::SubAssign for Coordinate {
    fn sub_assign(&mut self, other: Coordinate) {
        self.x -= other.x;
        self.y -= other.y;
    }
}

impl std::ops::Mul for Coordinate {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        Coordinate {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{super::Node, *};

    #[test]
    fn test_eq() {
        let co1: Coordinate = Coordinate::new(1, 1);
        let co2: Coordinate = co1.clone();
        let co3: Coordinate = Coordinate::new(2, 2);
        assert_eq!(co1, co2);
        assert_ne!(co1, co3);
        assert!(co1 < co3);
    }

    #[test]
    fn test_eq_macros() {
        let co1: Coordinate = Coordinate::new(1, 1);
        let co2: Coordinate = coordinate!(1);
        let co3: Coordinate = coordinate!(2);
        assert_eq!(co1, co2);
        assert_ne!(co1, co3);
        assert!(co1 < co3);
    }

    #[test]
    fn test_gen_within_radius() {
        // Default
        let co1: Coordinate = Coordinate::new(1, 1);
        // Get min and max points. Which the radius can't exceed.
        let co4: Coordinate = Coordinate::new(102, 102);
        let co5: Coordinate = Coordinate::new(-102, -102);

        // Since randomness is applied. It's effect is lowered by using many iterations.
        for _ in 0..20 {
            let co6: Coordinate = gen_within_radius(co1, 100);
            assert!(co4 > co6);
            assert!(co5 < co6);
        }
    }

    #[test]
    fn test_rotate_around_no_rotates() {
        let c1 = Coordinate::new(0, 0);
        for deg in 0..10 {
            let mut v = vec![Node::new("", Coordinate::new(0, 100))];
            rotate_around_axis(c1, &mut v, f64::from(deg * 360));
            assert_eq!(v.remove(0).geo, Coordinate::new(0, 100));
        }
    }

    #[test]
    fn test_rotate_around_one_circle() {
        let c1 = Coordinate::new(0, 0);
        let mut v = vec![Node::new("", Coordinate::new(0, 100))];
        for _ in 0..4 {
            rotate_around_axis(c1, &mut v, 90.0);
            println!("{:?}", v.get(0).unwrap().geo);
        }
        assert_eq!(v.remove(0).geo, Coordinate::new(0, 100));
    }

    #[test]
    fn test_rotate_around_radians_1() {
        let c1 = Coordinate::new(0, 0);
        let mut v = vec![Node::new("", Coordinate::new(0, 100))];
        rotate_around_axis(c1, &mut v, 90.0);
        assert_eq!(v.remove(0).geo, Coordinate::new(100, 0));
    }

    #[test]
    fn test_rotate_around_radians_2() {
        let c1 = Coordinate::new(0, 0);
        let mut v = vec![Node::new("", Coordinate::new(0, 100))];
        rotate_around_axis(c1, &mut v, 180.0);
        assert_eq!(v.remove(0).geo, Coordinate::new(0, -100));
    }

    #[test]
    fn test_rotate_around_radians_3() {
        let c1 = Coordinate::new(0, 0);
        let mut v = vec![Node::new("", Coordinate::new(0, 100))];
        rotate_around_axis(c1, &mut v, 270.0);
        assert_eq!(v.remove(0).geo, Coordinate::new(-100, 0));
    }

    #[test]
    fn test_rotate_around_moved_axis_1() {
        let c1 = Coordinate::new(100, 100);
        let mut v = vec![Node::new("", Coordinate::new(200, 100))];
        rotate_around_axis(c1, &mut v, 90.0);
        assert_eq!(v.remove(0).geo, Coordinate::new(100, 0));
    }

    #[test]
    fn test_rotate_around_moved_axis_2() {
        let c1 = Coordinate::new(99, 99);
        let mut v = vec![Node::new("", Coordinate::new(199, 99))];
        rotate_around_axis(c1, &mut v, 90.0);
        assert_eq!(v.remove(0).geo, Coordinate::new(99, -1));
    }

    #[test]
    fn test_gen_radius() {
        let co1: Coordinate = Coordinate::new(1, 1);
        let co4: Coordinate = Coordinate::new(102, 102);
        let co5: Coordinate = Coordinate::new(-102, -102);

        // Since randomness is applied. It's effect is lowered by using many iterations.
        for _ in 0..20 {
            let co6: Coordinate = gen_radius(co1, 0, 100);
            assert!(co4 > co6);
            assert!(co5 < co6);
        }
    }

    #[test]
    fn test_diff() {
        let co1: Coordinate = Coordinate::new(1, 1);
        let co2: Coordinate = Coordinate::new(102, 102);
        let co3: Coordinate = Coordinate::new(-102, -102);

        assert!(diff(co1, co2) == (101, 101));
        assert!(diff(co1, co3) == (103, 103));
        assert!(diff(co2, co3) == (204, 204));
        assert!(diff(co1, co1) == (0, 0));
    }

    #[test]
    fn test_diff_args_placement() {
        let co1: Coordinate = Coordinate::new(1, 1);
        let co2: Coordinate = Coordinate::new(102, 102);
        let co3: Coordinate = Coordinate::new(-102, -102);

        assert!(diff(co1, co2) == diff(co2, co1));
        assert!(diff(co1, co3) == diff(co3, co1));
        assert!(diff(co2, co3) == diff(co3, co2));
    }

    #[test]
    fn test_clone() {
        let co1: Coordinate = Coordinate::new(1, 1);
        let co2: Coordinate = Coordinate::new(9999, 9999);
        assert!(co1 == co1.clone());
        assert!(co2 == co2.clone());
    }

    #[test]
    fn test_clone_macros() {
        let co1: Coordinate = coordinate!();
        let co2: Coordinate = coordinate!(400);
        assert!(co1 == co1.clone());
        assert!(co2 == co2.clone());
    }
}
