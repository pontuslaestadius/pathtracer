extern crate rand;

use image::Rgba;
use node::coordinates::*;
use rand::distributions::{IndependentSample, Range};
use std::cmp::{max, min};
use std::f64;
use std::mem::swap;

/// Returns a random number between the min and maximum.
pub fn roll(min: u32, max: u32) -> u32 {
    let mut rng = rand::thread_rng();
    let between: Range<u32> = Range::new(min, max);
    between.ind_sample(&mut rng) as u32
}

/// Returns a random item from a given list.
pub fn get_random_item(list: &[String]) -> &String {
    let roll = roll(0, list.len() as u32);
    &list[roll as usize]
}

/// Tested
/// Checks so that the applied adjustments stay within a u8.
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

/// Returns a random Rgb color. the opacity is always 255.
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

/// Returns a random Rgb color. the opacity is always 255.
pub fn gen_rgba_reliable(seed: u64) -> Rgba<u8> {
    let seed = seed as u32;

    // Node
    let mut primary: Rgba<u8> = Rgba {data: [0,0,0,255]};

    let rem = 255;
    let mut min: u32 = (seed/2) % rem;
    let mut max: u32 = seed % rem;

    if min > max {
        swap(&mut min, &mut max);
    } else if min == max {
        min -=1;
    }

    // Color of the node.
    for i in 0..4 {
        let v = primary.data[i] as u32 + roll(min, max);

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
pub fn plot(coordinates1: &Coordinate, coordinates2: &Coordinate) -> Vec<Coordinate> {

    if coordinates1.x < 0 || coordinates2.x < 0 || coordinates1.y < 0 || coordinates2.y < 0 {
        panic!("Can't plot negative coordinates!");
    }

    let x0 = (coordinates1.x) as usize;
    let x1 = (coordinates2.x) as usize;

    let y0 = (coordinates1.y) as usize;
    let y1 = (coordinates2.y) as usize;

    // If it's a vertical line
    if coordinates1.x == coordinates2.x {

        let mut vec = Vec::new();
        for y in min(coordinates1.y,coordinates2.y)..max(coordinates1.y,coordinates2.y) {
            vec.push(Coordinate::new(coordinates1.x, y));
        }
        vec
        // If it's not a vertical line
    } else {
        plot_bresenham(x0, y0, x1, y1)
    }
}

/// Draws a line between two coordinate points.
/// https://en.wikipedia.org/wiki/Bresenham%27s_line_algorithm
/// Assumes that it is not vertical (deltaX != 0)
pub fn plot_bresenham(mut x0: usize, mut y0: usize, mut x1: usize, mut y1: usize) -> Vec<Coordinate> {

    // This case is handles reversed plotting, meaning going from a larger node to a smaller one.
    if (y0 < y1 && x0 > x1) || (y0 > y1 && x0 > x1) {
        swap(&mut x0, &mut x1);
        swap(&mut y0, &mut y1);
    }

    let delta_x: f64 = (x1 as i16 - x0 as i16) as f64; // TODO not pretty
    let delta_y: f64 = (y1 as i16 - y0 as i16) as f64; // TODO not pretty

    if delta_x == 0.00 {
        panic!("Bresenham does not support straight vertical lines!");
    }

    let delta_err: f64 = (delta_y / delta_x).abs();
    let mut error: f64 = 0.00;

    let mut y: i16 = y0 as i16;

    let mut plot: Vec<Coordinate> = Vec::new();
    let mut last_y = y;
    for x in min(x0, x1)..max(x0, x1)+1 {
        for i in min(last_y, y)..max(last_y, y)+1 {
            plot.push(Coordinate::new(x as i16, i));
        }
        last_y = y;

        error += delta_err;
        while error >= 0.50 {
            y += f64::signum(delta_y) as i16;
            error -= 1.00;
        }
    }
    plot
}
