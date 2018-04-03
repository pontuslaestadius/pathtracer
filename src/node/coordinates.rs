///
///    Coordinate
///    -----------
///    Stores an x and y coordinate.

extern crate rand;

use std::cmp::Ordering;
use std::f64;
use super::super::tools::roll;
use super::super::Coordinate;

// Constructs a randomly positioned coordinate.
pub fn gen() -> Coordinate {
    Coordinate {
        x: rand::random::<i16>(),
        y: rand::random::<i16>(),
    }
}

/// Get difference in distance.
pub fn diff(c1: &Coordinate, c2: &Coordinate) -> (i16, i16) {
    ((c1.x - c2.x).abs(), (c1.y - c2.y).abs())
}

/// Generate a Coordinate from a given Coordinate and randomly places it within a radius.
pub fn gen_within_radius(coord: &Coordinate, radius: u32) -> Coordinate {
    gen_radius(&coord, 0, radius)
}

/// Generate a Coordinate from a given Coordinate and randomly places it within a min and max radius.
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
