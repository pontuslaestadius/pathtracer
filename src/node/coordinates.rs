///
///    Coordinate
///    -----------
///    Stores an x and y coordinate.

extern crate rand;

use std::cmp::Ordering;
use std::f64;
use super::util::roll;

#[derive(Eq)]
pub struct Coordinate {
    pub x: i16,
    pub y: i16,
}

impl Coordinate {

    // Nothing to test.
    pub fn new(x: i16, y: i16) -> Coordinate {
        Coordinate {
            x,
            y
        }
    }

    // Nothing to test.
    pub fn gen() -> Coordinate {
        Coordinate {
            x: rand::random::<i16>(),
            y: rand::random::<i16>(),
        }
    }

    // Transitively tested.
    pub fn diff(&self, other: &Coordinate) -> (i16, i16) {
        diff(&self, other)
    }

}

/// Tested
// Get difference in distance.
pub fn diff(c1: &Coordinate, c2: &Coordinate) -> (i16, i16) {
    ((c1.x - c2.x).abs(), (c1.y - c2.y).abs())
}

/// Tested
/// Generate a Coordinate from a given Coordinate and randomly places it within a radius.
pub fn gen_within_radius(coord: &Coordinate, radius: u32) -> Coordinate {
    gen_radius(&coord, 0, radius)
}

/// Tested
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

/// Tested
impl Ord for Coordinate {
    fn cmp(&self, other: &Coordinate) -> Ordering {
        (self.x + self.y).cmp(&(&other.x + &other.y))  // TODO improve.
    }
}

/// Tested
impl PartialOrd for Coordinate {
    fn partial_cmp(&self, other: &Coordinate) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

/// Tested
impl Clone for Coordinate {
    fn clone(&self) -> Coordinate {
        Coordinate {
            x: self.x,
            y: self.y
        }
    }
}

// Tested
impl PartialEq for Coordinate {
    fn eq(&self, other: &Coordinate) -> bool {
        (self.x == other.x) && (self.y == other.y)
    }
}
