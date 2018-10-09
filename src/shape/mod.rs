use super::{tools::plot, Coordinate, Shape};

#[derive(Debug, Clone)]
pub struct Square {}

#[derive(Debug, Clone)]
pub struct Circle {}

#[derive(Debug, Clone)]
pub struct Triangle {}

impl Shape for Square {
    fn new() -> Square { Square {} }

    /// Returns all coordinates that the shape occupies.
    /// Assume that you start at coordinate x: 0, y: 0.
    fn area(&self, size: u32) -> Vec<Coordinate> {
        let mut vec = Vec::new();
        for i in 0..size {
            for j in 0..size {
                vec.push(Coordinate::new(i as i16, j as i16));
            }
        }
        vec
    }
}

impl Shape for Circle {
    fn new() -> Circle { Circle {} }

    /// Returns all coordinates that the shape occupies.
    /// Algorithm is derived from:
    /// https://en.wikipedia.org/wiki/Midpoint_circle_algorithm
    fn area(&self, size: u32) -> Vec<Coordinate> {
        let mut vec = Vec::new();

        let mut x: i16 = (size - 1) as i16;
        let mut y: i16 = 0;
        let mut dx: i16 = 1;
        let mut dy: i16 = 1;
        let x0: i16 = 0;
        let y0: i16 = 0;
        let mut err: i16 = dx - (size << 1) as i16;

        let q_plot = |x1, y1, x2, y2| plot(Coordinate::new(x1, y1), Coordinate::new(x2, y2));

        while x >= y {
            vec.append(&mut q_plot(x0 + x, y0 + y, x0 - x, y0 + y));
            vec.append(&mut q_plot(x0 + x, y0 - y, x0 - x, y0 - y));
            vec.append(&mut q_plot(x0 - y, y0 - x, x0 - y, y0 + x));
            vec.append(&mut q_plot(x0 + y, y0 - x, x0 + y, y0 + x));

            if err <= 0 {
                y += 1;
                err += dy;
                dy += 2;
            } else {
                x -= 1;
                dx += 2;
                err += dx - (size << 1) as i16;
            }
        }

        vec
    }
}

impl Shape for Triangle {
    fn new() -> Triangle { Triangle {} }

    /// Returns all coordinates that the shape occupies.
    /// Assume that you start at coordinate x: 0, y: 0.
    fn area(&self, size: u32) -> Vec<Coordinate> {
        let mut vec = Vec::new();
        let size = size as i16;
        let start_x = size / 2;

        for i in 0..size {
            vec.append(&mut plot(
                Coordinate::new(start_x, 0),
                Coordinate::new(i, size),
            ));
        }
        vec
    }
}
