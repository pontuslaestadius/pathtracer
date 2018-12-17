extern crate image;
extern crate rand;

use super::{Coordinate, Hash};
use image::Rgba;
use rand::{distributions::Uniform, Rng};

use std::{
    cmp::{max, min},
    f64,
    mem::swap,
};

/// Finds an element using their hashes.
pub fn find<T: Hash>(element: u64, list: &[T]) -> Option<&T> {
    list.iter().find(|&x| x.hash() == element)
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
/// # let color = image::Rgba([100, 100, 100, 255]);
/// let base = Coordinate::new(0, 0);
/// let to = Coordinate::new(10, 10);
/// assert_eq!(
///     tools::range_color(falloff, color, base, to),
///     image::Rgba([77, 77, 77, 255])
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

    image::Rgba([
        border(base[0], modify),
        border(base[1], modify),
        border(base[2], modify),
        border(base[3], 0),
    ])
}

/// Returns a random number between the min and maximum.
/// # Examples
/// ```
/// # use pathfinder::tools;
/// let nr = tools::roll(50u32, 60);
/// assert!(nr >= 50 && nr <= 60);
/// ```
pub fn roll<T: Into<u32>>(min: T, max: T) -> u32 {
    let mut rng = rand::thread_rng();
    rng.sample(Uniform::new(min.into(), max.into()))
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
/// assert_eq!(tools::border(a, b), 255);
/// assert_eq!(tools::border(a, -b), 0);
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
    (0..4).fold(super::consts::DEFAULT_RGBA, |mut acc, x| {
        acc.data[x] = acc.data[x].saturating_add(roll(0u8, u8::max_value()) as u8);
        acc
    })
}

/// Returns a Rgb color based on a seed value. the opacity is always 255.
pub fn seed_rgba(seed: u64) -> Rgba<u8> {
    let r = seed % 254;
    let g = (seed + 75) % 254;
    let b = (seed + 150) % 254;

    Rgba([r as u8, g as u8, b as u8, 255])
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
pub fn plot(a: Coordinate, b: Coordinate) -> Vec<Coordinate> { plot_type(a, b, &plot_bresenham) }

pub fn plot_type(
    mut a: Coordinate,
    mut b: Coordinate,
    plot_kind: &Fn(Coordinate, Coordinate) -> Vec<Coordinate>,
) -> Vec<Coordinate> {
    // If any of the coordinates are negative, interally add to make them positive.
    if a.lt(0) || b.lt(0) {
        let add = Coordinate::new(max(-a.x, -b.x), max(-a.y, -b.y));
        plot_type(a + add, b + add, &plot_bresenham)
            .iter()
            .fold(vec![], |mut acc, c| {
                acc.push(*c - add);
                acc
            })
    // If it's a vertical line
    } else if a.x == b.x {
        (min(a.y, b.y)..max(a.y, b.y))
            .map(|y| Coordinate::new(a.x, y))
            .collect()
    } else {
        if b.x < a.x {
            swap(&mut a, &mut b);
        }
        plot_kind(a, b)
    }
}

/// Draws a line between two coordinate points.
/// Derived from: https://en.wikipedia.org/wiki/Bresenham%27s_line_algorithm
/// # Panics
///
/// delta_x == 0.00
pub fn plot_bresenham(mut from: Coordinate, to: Coordinate) -> Vec<Coordinate> {
    let delta = to - from;
    let delta_x = f64::from(delta.x);
    let delta_y = f64::from(delta.y);

    if delta_x == 0.00 {
        panic!("Bresenham does not support straight vertical lines!");
    }

    let delta_err: f64 = (delta_y / delta_x).abs();
    let mut error: f64 = 0.00;

    let mut plot: Vec<Coordinate> = Vec::new();
    let mut last_y = from.y;

    for x in min(from.x, to.x)..=max(from.x, to.x) {
        for y in min(last_y, from.y)..=max(last_y, from.y) {
            plot.push(Coordinate::new(x as i16, y));
        }
        last_y = from.y;
        error += delta_err;
        while error >= 0.50 {
            from.y += f64::signum(delta_y) as i16;
            error -= 1.00;
        }
    }
    plot
}

/// Draws a line between two coordinate points in straight horizontal or
/// vertical lines.
pub fn plot_rectangle(mut from: Coordinate, to: Coordinate) -> Vec<Coordinate> {
    let mut plot: Vec<Coordinate> = Vec::new();
    let mut delta = Coordinate::new(1, 1);

    if from.x > to.x {
        delta.x = -1;
    }

    if from.y > to.y {
        delta.y = -1;
    }

    while from.x != to.x {
        from.x += delta.x;
        plot.push(from);
    }

    while from.y != to.y {
        from.y += delta.y;
        plot.push(from);
    }

    plot
}

/// Gives the midpoint between two points.
/// # Examples
/// First declare the colors and the range at which it becomes darker.
/// ```
/// use pathfinder::{tools, Coordinate};
/// let a = Coordinate::new(0, 0);
/// let b = Coordinate::new(100, 100);
/// let mid = tools::midpoint(a, b);
/// assert_eq!(mid, Coordinate::new(50, 50));
/// ```
pub fn midpoint(a: Coordinate, b: Coordinate) -> Coordinate {
    Coordinate::new((a.x + b.x) / 2, (a.y + b.y) / 2)
}

/// # Experimental! Work in progress.
/// Draws a line between two coordinate points in the form on a ellipse.
/// Derived from: https://en.wikipedia.org/wiki/Ellipse
pub fn plot_ellipse(mut from: Coordinate, to: Coordinate) -> Vec<Coordinate> {
    let min = Coordinate::new(min(from.x, to.x), min(from.y, to.y));
    let _max = Coordinate::new(max(from.x, to.x), max(from.y, to.y));
    let _t_max = midpoint(from, to);

    let mut result = Vec::new();
    let t = f64::consts::PI / 2.0; // How the ellipse is angled. and how steep it angles.
    let mut theta: f64 = 0.5; // starting angle.
    let diff = from - to;
    let step: f64 = t / 4.5; // Number of ellipse steps from start to finish.
    let c = min;
    let r: f64 = f64::from(diff.x / 2).abs(); // distance from center to node aka radius.

    println!(
        "r: {} |t: {} |s: {} |from: {} |to: {} |diff: {} |c: {}",
        r, t, step, from, to, diff, c
    );

    // -r = J shape. r = n shape.
    let mut s = Coordinate::new(1, 1);
    if from.y > to.y {
        s.y = -1;
    }

    while t > theta {
        let point = from
            + Coordinate::new(
                (r * theta.cos()) as i16,
                (f64::from(s.y) * r * theta.sin() / 2.0) as i16,
            );
        println!("Theta: {} | Coordinate: {} <- {}", theta, point, from);
        let mut line = plot_type(from, point, &plot_bresenham);
        result.append(&mut line);
        from = point;
        theta += step;
    }
    result.append(&mut plot_type(from, to, &plot_bresenham));
    //println!("{:#?}", result);
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_border() {
        assert_eq!(border(0, 0), 0);
        assert_eq!(border(0, -55), 0);
        assert_eq!(border(100, 100), 200);
        assert_eq!(border(255, 255), 255);
        assert_eq!(border(0, 255), 255);
        assert_eq!(border(255, 0), 255);
        assert_eq!(border(255, -255), 0);
    }

    #[test]
    fn test_roll() {
        let res = roll(0u32, 5);
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
