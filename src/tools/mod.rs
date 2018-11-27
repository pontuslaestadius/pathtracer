extern crate image;
extern crate rand;

use super::{Coordinate, Hash};
use image::Rgba;
use rand::distributions::{IndependentSample, Range};
use std::{
    cmp::{max, min},
    f64,
    mem::swap,
};

/// Finds an element using their hashes.
pub fn find<T: Hash>(element: u64, list: &[T]) -> Option<&T> {
    for l in list {
        if element == l.hash() {
            return Some(l);
        }
    }
    None
}

/// Returns a Rgba with a modified value depending on how close it is to it's
/// falloff point.
///
/// # Examples
/// First declare the colors and the range at which it becomes darker.
/// ```
/// extern crate image;
/// # extern crate pathfinder;
/// use pathfinder::{tools, Coordinate};
///
/// let falloff = 100;
/// let color = image::Rgba {
///     data: [100, 100, 100, 255],
/// };
/// ```
/// Evaluate that based on the modified distance, in the small sense it is
/// modified from the defined color above.
/// ```
/// # extern crate image;
/// # extern crate pathfinder;
/// # use pathfinder::{tools, Coordinate};
///
/// # let falloff = 100;
/// # let color = image::Rgba {
/// #     data: [100, 100, 100, 255],
/// # };
/// let base = Coordinate::new(0, 0);
/// let to = Coordinate::new(10, 10);
/// assert_eq!(
///     tools::range_color(falloff, color, base, to),
///     image::Rgba {
///         data: [77, 77, 77, 255]
///     }
/// );
/// ```
pub fn range_color(
    falloff: i16,
    base: image::Rgba<u8>,
    base_geo: Coordinate,
    to_geo: Coordinate,
) -> image::Rgba<u8> {
    let (x_dif, y_dif) = base_geo.diff(&to_geo);
    let x_scale: f64 = f64::from(x_dif) / f64::from(falloff);
    let y_scale: f64 = f64::from(y_dif) / f64::from(falloff);
    let max_multi: f64 =
        f64::from(i32::from(base[0]) + i32::from(base[1]) + i32::from(base[2]) / 3);
    let modify = (-max_multi * (x_scale + y_scale) / 2.0) as i32;
    image::Rgba {
        data: [
            border(base[0], modify),
            border(base[1], modify),
            border(base[2], modify),
            border(base[3], 0),
        ],
    }
}

/// Returns a random number between the min and maximum.
/// # Examples
/// ```
/// # use pathfinder::tools;
/// let nr = tools::roll(50, 60);
/// assert!(nr >= 50 && nr <= 60);
/// ```
pub fn roll(min: u32, max: u32) -> u32 {
    let mut rng = rand::thread_rng();
    let between: Range<u32> = Range::new(min, max);
    between.ind_sample(&mut rng) as u32
}

/// Returns a random item from a given list.
pub fn random_item(list: &[String]) -> &String {
    let roll = roll(0, list.len() as u32);
    &list[roll as usize]
}

/// Checks so that the applied adjustments stay within a u8.
/// # Examples
/// ```
/// use pathfinder::tools;
/// let a = 50;
/// let b = 250;
/// let max = tools::border(a, b);
/// let zero = tools::border(a, -b);
/// assert_eq!(max, 255);
/// assert_eq!(zero, 0);
/// ```
pub fn border(a: u8, b: i32) -> u8 { max(min(i32::from(a) + b, 255 as i32), 0) as u8 }

/// Returns a random Rgb color. the opacity is always 255.
/// # Examples
/// ```
/// # use pathfinder::tools;
/// let rgba = tools::gen_rgba();
/// println!("{:?}", rgba.data);
/// ```
pub fn gen_rgba() -> Rgba<u8> {
    let mut primary: Rgba<u8> = super::consts::DEFAULT_RGBA;

    for i in 0..4 {
        let v = u32::from(primary.data[i]) + roll(0, 255);
        primary.data[i] = min(v, 255) as u8;
    }

    primary
}

/// Returns a Rgb color based on a seed value. the opacity is always 255.
pub fn seed_rgba(seed: u64) -> Rgba<u8> {
    let r = seed % 254;
    let g = (seed + 75) % 254;
    let b = (seed + 150) % 254;

    Rgba {
        data: [r as u8, g as u8, b as u8, 255],
    }
}

/// Implemented according to
/// https://en.wikipedia.org/wiki/Bresenham%27s_line_algorithm
///
/// Plots the pixels between two coordinate points.
/// Checks so that the applied adjustments stay within a u8.
///
/// # Examples
///
/// First we create a simple path between two coordinates.
///
/// ```
/// use pathfinder::{tools, Coordinate};
/// let a = Coordinate::new(0, 0);
/// let b = Coordinate::new(1, 1);
/// let path = tools::plot(a, b);
/// ```
///
/// To confirm the small path created. Larger paths are more difficult to
/// manually test.
///
/// ```
/// # use pathfinder::{tools, Coordinate};
/// # let a = Coordinate::new(0, 0);
/// # let b = Coordinate::new(1, 1);
/// # let path = tools::plot(a, b);
/// let correct_path = vec![
///     Coordinate::new(0, 0),
///     Coordinate::new(1, 0),
///     Coordinate::new(1, 1),
/// ];
/// assert_eq!(path, correct_path);
/// ```
pub fn plot(a: Coordinate, b: Coordinate) -> Vec<Coordinate> {
    // If any of the coordinates are negative, interally add to make them positive.
    if a.x < 0 || b.x < 0 || a.y < 0 || b.y < 0 {
        let add = Coordinate::new(max(-a.x, -b.x), max(-a.y, -b.y));
        let mut vec_final = Vec::new();
        for c in &plot(a + add, b + add) {
            vec_final.push(*c - add);
        }
        return vec_final;
    }

    // If it's a vertical line
    if a.x == b.x {
        (min(a.y, b.y)..max(a.y, b.y))
            .map(|y| Coordinate::new(a.x, y))
            .collect()

    // If it's not a vertical line
    } else {
        plot_bresenham(a.x as usize, a.y as usize, b.x as usize, b.y as usize)
    }
}

/// Draws a line between two coordinate points.
/// Derived from: https://en.wikipedia.org/wiki/Bresenham%27s_line_algorithm
/// # Panics
///
/// delta_x == 0.00
fn plot_bresenham(mut x0: usize, mut y0: usize, mut x1: usize, mut y1: usize) -> Vec<Coordinate> {
    // This case is handles reversed plotting, meaning going from a larger node to
    // a smaller one.
    if y0 != y1 && x0 > x1 {
        swap(&mut x0, &mut x1);
        swap(&mut y0, &mut y1);
    }

    // Convert the numbers to be isize to enable the usize values to be less than 0.
    let delta_x: f64 = (x1 as isize - x0 as isize) as f64;
    let delta_y: f64 = (y1 as isize - y0 as isize) as f64;

    if delta_x == 0.00 {
        panic!("Bresenham does not support straight vertical lines!");
    }

    let delta_err: f64 = (delta_y / delta_x).abs();
    let mut error: f64 = 0.00;

    let mut y: i16 = y0 as i16;

    let mut plot: Vec<Coordinate> = Vec::new();
    let mut last_y = y;
    for x in min(x0, x1)..=max(x0, x1) {
        for i in min(last_y, y)..=max(last_y, y) {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_border_zero() {
        assert_eq!(border(0, 0), 0);
    }

    #[test]
    fn test_border_negative_adjust() {
        assert_eq!(border(0, -55), 0);
    }

    #[test]
    fn test_border_sub_max_add() {
        assert_eq!(border(100, 100), 200);
    }

    #[test]
    fn test_border_set_max() {
        assert_eq!(border(0, 255), 255);
    }

    #[test]
    fn test_border_too_high() {
        assert_eq!(border(255, 255), 255);
    }

    #[test]
    fn test_border_no_add() {
        assert_eq!(border(255, 0), 255);
    }

    #[test]
    fn test_border_eq() {
        assert_eq!(border(255, -255), 0);
    }

    #[test]
    fn test_roll() {
        let res = roll(0, 5);
        assert!(res <= 5);
    }

    #[test]
    fn test_random_item() {
        let strings = ["a".to_string(), "b".to_string()];
        let res = random_item(&strings);
        let res = res == &strings[0] || res == &strings[1];
        assert!(res);
    }

    #[test]
    fn test_gen_rgba() {
        for _ in 0..5 {
            let _ = gen_rgba();
        }
    }

    #[test]
    fn test_seed_rgba() {
        let seed = 9611;
        for i in 0..30 {
            let _ = seed_rgba(seed * i);
        }
    }

    #[test]
    fn test_seed_rgba_consistancy() {
        let seed = 9611;
        let res = seed_rgba(seed);
        for _ in 0..30 {
            assert_eq!(seed_rgba(seed), res);
        }
    }

    #[test]
    fn test_plot_start_and_end() {
        let c1 = Coordinate::new(0, 0);
        let c2 = Coordinate::new(10, 10);
        let plot = plot(c1, c2);
        assert!(c1 == plot[0] && c2 == plot[plot.len() - 1]);
    }

    #[test]
    fn test_plot_straight_line() {
        let c1 = Coordinate::new(0, 0);
        let c2 = Coordinate::new(0, 10);
        let plot = plot(c1, c2);
        for i in 0..10 {
            assert_eq!(plot[i].y, i as i16);
        }
    }
}
