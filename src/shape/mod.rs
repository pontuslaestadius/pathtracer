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
    /// Assume that 0 0 is the center of the node.
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
        let mut d = Coordinate::new(1, 1);
        let mut err: i16 = d.x - (size << 1) as i16;

        let q_plot = |x1, y1, x2, y2| plot(Coordinate::new(x1, y1), Coordinate::new(x2, y2));

        while x >= y {
            vec.append(&mut q_plot(x, y, -x, y));
            vec.append(&mut q_plot(x, -y, -x, -y));
            vec.append(&mut q_plot(-y, -x, -y, x));
            vec.append(&mut q_plot(y, -x, y, x));

            if err <= 0 {
                y += 1;
                err += d.y;
                d.y += 2;
            } else {
                x -= 1;
                d.x += 2;
                err += d.x - (size << 1) as i16;
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
        for i in 0..size {
            vec.append(&mut plot(
                Coordinate::new(size / 2, 0),
                Coordinate::new(i, size),
            ));
        }
        vec
    }
}
