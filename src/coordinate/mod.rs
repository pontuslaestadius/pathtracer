extern crate pythagoras;
extern crate rand;

use super::{tools::roll, Coordinate};
use std::{cmp::Ordering, f64, ops::Add};

/// Constructs a vector of generic structs from a given list convered to
/// Coordinates.
pub fn from_list<T>(list: &[(i16, i16)], get: &Fn(Coordinate, usize) -> T) -> Vec<T> {
    let mut result: Vec<T> = Vec::new();
    for (i, &(x, y)) in list.iter().enumerate() {
        result.push(get(Coordinate::new(x, y), i));
    }
    result
}

/// Constructs a randomly positioned coordinate.
pub fn gen() -> Coordinate {
    Coordinate {
        x: rand::random::<i16>(),
        y: rand::random::<i16>(),
    }
}

/// Get a coordinate based of a function.
///
/// # Examples
/// ```
/// use pathfinder::{coordinate::*, Coordinate};
/// let c1 = Coordinate::new(0, 0);
/// let f = |i: usize| -> Coordinate { Coordinate::new(i as i16, i as i16) };
/// let c2 = calc(c1, 5, &f);
/// assert_eq!(c2, Coordinate { x: 5, y: 5 });
/// ```
pub fn calc(start: Coordinate, variable: usize, call: &Fn(usize) -> Coordinate) -> Coordinate {
    let res = call(variable);
    Coordinate {
        x: start.x + res.x,
        y: start.y + res.y,
    }
}

/// Get difference in distance.
///
/// # Examples
/// ```
/// use pathfinder::{coordinate::*, Coordinate};
/// let c1 = Coordinate::new(0, 0);
/// let c2 = Coordinate::new(100, 100);
/// let difference = c1.diff(c2);
/// assert_eq!(difference, (100, 100));
/// ```
pub fn diff(c1: Coordinate, c2: Coordinate) -> (i16, i16) {
    ((c1.x - c2.x).abs(), (c1.y - c2.y).abs())
}

/// Get the distance between two Coordinates'.
///
/// # Examples
/// ```
/// use pathfinder::{coordinate::distance, Coordinate};
/// let a = Coordinate::new(0, 0);
/// let b = Coordinate::new(3, 4);
/// let distance = distance(a, b);
/// assert_eq!(distance, 5);
/// ```
pub fn distance(a: Coordinate, b: Coordinate) -> u32 {
    let diff = diff(a, b);
    pythagoras::theorem(diff.0, diff.1) as u32
}

/// Generate a Coordinate from a given Coordinate and randomly places it within
/// a radius.
///
/// # Examples
/// ```
/// use pathfinder::{coordinate::*, Coordinate};
/// let c1 = Coordinate::new(0, 0);
/// let c2 = gen_within_radius(c1, 100);
/// ```
pub fn gen_within_radius(coord: Coordinate, radius: u32) -> Coordinate {
    gen_radius(coord, 0, radius)
}

/// Generate a Coordinate from a given Coordinate and randomly places it within
/// a min and max radius.
///
/// # Examples
/// ```
/// use pathfinder::{coordinate::*, Coordinate};
/// let c1 = Coordinate::new(0, 0);
/// let c2 = gen_radius(c1, 50, 100);
/// ```
pub fn gen_radius(coord: Coordinate, min: u32, max: u32) -> Coordinate {
    // Randomly gets the radius of the circle.
    let r = f64::from(roll(min, max));

    // gets a point on the circle's circumference.
    let circle = |a: f64, b: f64| a + r * b;

    // Gets a random angle.
    let angle = roll(0, 3600);
    let a: f64 = f64::consts::PI * 0.001 * f64::from(angle);

    let x = circle(f64::from(coord.x), a.cos()) as i16;
    let y = circle(f64::from(coord.y), a.sin()) as i16;

    Coordinate { x, y }
}

/// Rotates the provide Vec around the axis inplace.
///
/// # Examples
/// ```
/// use pathfinder::{coordinate::*, Coordinate, Node};
/// let c1 = Coordinate::new(0, 0);
/// let n = Node::new("", Coordinate::new(0, 100));
/// let mut v = vec!(n);
/// rotate_around_axis(c1, &mut v, 1.0);
/// assert_eq!(v.remove(0).geo, Coordinate::new(99, 1));
/// ```
pub fn rotate_around_axis(axis: Coordinate, points: &mut Vec<super::Node>, rad: f64) {
    let rad = rad * f64::consts::PI / 180.0;
    for mut p in points.iter_mut() {
        let radius = distance(axis, p.geo) as f64;
        let diff = diff(p.geo, axis);
        let base = (diff.0 as f64).atan2(diff.1 as f64);
        let angle: f64 = base + rad;
        p.geo.x = axis.x + (angle.cos()*radius) as i16;
        p.geo.y = axis.y + (angle.sin()*radius) as i16;
    }
}

impl Ord for Coordinate {
    fn cmp(&self, other: &Coordinate) -> Ordering { (self.x + self.y).cmp(&(other.x + other.y)) }
}

impl PartialOrd for Coordinate {
    fn partial_cmp(&self, other: &Coordinate) -> Option<Ordering> { Some(self.cmp(other)) }
}

impl PartialEq for Coordinate {
    fn eq(&self, other: &Coordinate) -> bool { (self.x == other.x) && (self.y == other.y) }
}

impl Add for Coordinate {
    type Output = Coordinate;

    fn add(self, other: Coordinate) -> Coordinate {
        Coordinate::new(self.x + other.x, self.y + other.y)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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

        assert!(co1.diff(co2) == (101, 101));
        assert!(co1.diff(co3) == (103, 103));
        assert!(co2.diff(co3) == (204, 204));
        assert!(co1.diff(co1) == (0, 0));
    }

    #[test]
    fn test_clone() {
        let co1: Coordinate = Coordinate::new(1, 1);
        let co2: Coordinate = Coordinate::new(9999, 9999);
        assert!(co1 == co1.clone());
        assert!(co2 == co2.clone());
    }
}
