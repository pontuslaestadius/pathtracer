extern crate rand;
extern crate pythagoras;

use std::cmp::Ordering;
use std::f64;
use super::super::tools::roll;
use super::super::Coordinate;
use std::ops::Add;

// Constructs a randomly positioned coordinate.
pub fn gen() -> Coordinate {
    Coordinate {
        x: rand::random::<i16>(),
        y: rand::random::<i16>(),
    }
}

/// Get difference in distance.
/// # Examples
/// ```
/// use pathfinder::Coordinate;
/// use pathfinder::node::coordinates::*;
/// let c1 = Coordinate::new(0,0);
/// let c2 = Coordinate::new(100,100);
/// let difference = c1.diff(&c2);
/// assert_eq!(difference, (100, 100));
/// ```
pub fn diff(c1: &Coordinate, c2: &Coordinate) -> (i16, i16) {
    ((c1.x - c2.x).abs(), (c1.y - c2.y).abs())
}

/// Get the distance between two Coordinates'.
/// # Examples
/// ```
/// use pathfinder::Coordinate;
/// use pathfinder::node::coordinates::distance;
/// let a = Coordinate::new(0, 0);
/// let b = Coordinate::new(3,4);
/// let distance = distance(&a, &b);
/// assert_eq!(distance, 5);
/// ```
pub fn distance(a: &Coordinate, b: &Coordinate) -> u32 {
    let diff = diff(a, b);
    pythagoras::theorem(diff.0, diff.1) as u32
}

/// Generate a Coordinate from a given Coordinate and randomly places it within a radius.
/// # Examples
/// ```
/// use pathfinder::Coordinate;
/// use pathfinder::node::coordinates::*;
/// let c1 = Coordinate::new(0,0);
/// let c2 = gen_within_radius(&c1, 100);
/// ```
pub fn gen_within_radius(coord: &Coordinate, radius: u32) -> Coordinate {
    gen_radius(&coord, 0, radius)
}

/// Generate a Coordinate from a given Coordinate and randomly places it within a min and max radius.
/// # Examples
/// ```
/// use pathfinder::Coordinate;
/// use pathfinder::node::coordinates::*;
/// let c1 = Coordinate::new(0,0);
/// let c2 = gen_radius(&c1, 50, 100);
/// ```
pub fn gen_radius(coord: &Coordinate, min: u32, max: u32) -> Coordinate {
    // Randomly gets the radius of the circle.
    let r = roll(min, max) as f64;

    // gets a point on the circle's circumference.
    let circle = |a: f64, b: f64| a + r * b;

    // Gets a random angle.
    let angle = roll(0, 3600);
    let a: f64 = f64::consts::PI * (0.001 * angle as f64);

    // x = cx + r * cos(a)
    let x = circle(coord.x as f64, a.cos()) as i16;
    // y = cy + r * sin(a)
    let y = circle(coord.y as f64, a.sin()) as i16;

    Coordinate {
        x,
        y
    }
}

impl Ord for Coordinate {
    fn cmp(&self, other: &Coordinate) -> Ordering {
        (self.x + self.y).cmp(&(&other.x + &other.y))  // TODO improve.
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

impl Add for Coordinate {
    type Output = Coordinate;
    fn add(self, other: Coordinate) -> Coordinate {
        Coordinate::new(self.x + other.x, self.y + other.y)
    }
}