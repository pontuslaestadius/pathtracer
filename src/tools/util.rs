extern crate rand;
use super::constants;
use rand::distributions::{IndependentSample, Range};
use image::Rgba;
use node::coordinates::*;

use std::cmp::{min, max};
use std::f64;
use std::mem::swap;

// Standard println with an applied condition.
pub fn debug_print(str: &str) {
    if constants::DEBUGMODE {
        println!("{}", str);
    }
}

// Returns a random number between the min and maximum.
pub fn roll(min: u32, max: u32) -> u32 {
    let mut rng = rand::thread_rng();
    let between: Range<u32> = Range::new(min, max);
    between.ind_sample(&mut rng) as u32
}

// Returns a random item from a given list.
pub fn get_random_item(list: &[String]) -> &String {
    let roll = roll(0, list.len() as u32);
    &list[roll as usize]
}

// Checks so that the applied adjustments stay within a u8.
pub fn border(a: u8, b: i32) -> u8 {
    let a = a as i32;

    // If it's too big.
    if a+b > 255 {
        255 as u8
        // If it's too small.
    } else if a+b < 0 {
        0 as u8
        // If it's alright.
    } else {
        (a+b) as u8
    }
}

// Returns a random Rgb color. the opacity is always 255.
pub fn gen_rgba() -> Rgba<u8> {

    // Node
    let mut primary: Rgba<u8> = Rgba {data: [0,0,0,255]};

    // Color of the node.
    for i in 0..4 {
        let v = primary.data[i] as u32 + roll(0,255);

        // If v goes above what a u8 can take. Set it to max.
        let v2 = if v > 255 {
            255
        } else {
            v
        };

        primary.data[i] = v2 as u8;
    }

    primary
}


/// https://en.wikipedia.org/wiki/Bresenham%27s_line_algorithm
/// Plots the pixels between two coordinate points.
/// Both coordinates must be > 0.
pub fn plot(coordinates1: &Coordinates, coordinates2: &Coordinates) -> Vec<Coordinates> {

    if coordinates1.x < 0 || coordinates2.x < 0 || coordinates1.y < 0 || coordinates2.y < 0 {
        panic!("Can't plot negative coordinates!");
    }

    let mut x0 = (coordinates1.x) as usize;
    let mut x1 = (coordinates2.x) as usize;

    let mut y0 = (coordinates1.y) as usize;
    let mut y1 = (coordinates2.y) as usize;

    // If it's a vertical line
    if coordinates1.x == coordinates2.x {

        let mut vec = Vec::new();
        for y in min(coordinates1.y,coordinates2.y)..max(coordinates1.y,coordinates2.y) {
            vec.push(Coordinates::new(coordinates1.x, y));
        }
        vec
        // If it's not a vertical line
    } else {
        // Swap the values.
        if x0 > x1 {
            swap(&mut x0, &mut x1);
            swap(&mut y0, &mut y1);
        }

        plot_bresenham(x0, y0, x1, y1)
    }
}

/// Draws a line between two coordinate points.
/// https://en.wikipedia.org/wiki/Bresenham%27s_line_algorithm
/// Assumes the following:
///      Not vertical (deltaX != 0)
///      x0 < x1 and y0 < y1
pub fn plot_bresenham(x0: usize, y0: usize, x1: usize, y1: usize) -> Vec<Coordinates> {
    let delta_x: f64 = (x1 as i16 - x0 as i16) as f64;
    let delta_y: f64 = (y1 as i16 - y0 as i16) as f64;

    if delta_x == 0.00 {
        panic!("Bresenham does not support straight vertical lines!");
    } else if delta_x < 0.00 {
        panic!("Bresenham does not support negative delta x!");
    }

    let delta_err: f64 = (delta_y / delta_x).abs();
    let mut error: f64 = 0.00;

    let mut y = y0;

    let mut plot: Vec<Coordinates> = Vec::new();
    for x in x0..x1 {
        plot.push(Coordinates::new(x as i16, y as i16));
        error += delta_err;
        while error >= 0.50 {
            y += (f64::signum(delta_y) *1.00) as usize;
            error -= 1.00;
        }
    }
    plot
}


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