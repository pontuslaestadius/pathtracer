/*!
Shapes used to calculate areas.
 */

use super::{tools::plot, Coordinate, Shape};

#[derive(Debug, Clone)]
pub struct Square {}

#[derive(Debug, Clone)]
pub struct Circle {}

#[derive(Debug, Clone)]
pub struct Triangle {}

impl Shape for Square {
    fn new() -> Square { Square {} }

    /**
     Returns all coordinates that the shape occupies.
     Assume that 0 0 is the center of the node.
    */
    fn area(&self, size: u32) -> Vec<Coordinate> {
        let size = size as i16;
        let mut vec = Vec::new();
        for i in 0i16..size {
            for j in 0i16..size {
                vec.push(Coordinate::new(i, j));
            }
        }
        vec
    }
}

impl Shape for Circle {
    fn new() -> Circle { Circle {} }

    /**
        Returns all coordinates that the shape occupies.
     Algorithm is derived from:
    https://en.wikipedia.org/wiki/Midpoint_circle_algorithm
    */
    fn area(&self, size: u32) -> Vec<Coordinate> {
        let mut vec = Vec::new();
        let mut pos = Coordinate::new((size - 1) as i16, 0);
        let mut err: i16 = 1 - (size << 1) as i16;
        let mut d = Coordinate::new(err, 1);

        let q_plot = |x1, y1, x2, y2| plot(Coordinate::new(x1, y1), Coordinate::new(x2, y2));

        while pos.x >= pos.y {
            vec.append(&mut q_plot(pos.x, pos.y, -pos.x, pos.y));
            vec.append(&mut q_plot(pos.x, -pos.y, -pos.x, -pos.y));
            vec.append(&mut q_plot(-pos.y, -pos.x, -pos.y, pos.x));
            vec.append(&mut q_plot(pos.y, -pos.x, pos.y, pos.x));

            if err <= 0 {
                pos.y += 1;
                d.y += 2;
                err += d.y;
            } else {
                pos.x -= 1;
                d.x += 2;
                err += d.x;
            }
        }

        vec
    }
}

impl Shape for Triangle {
    fn new() -> Triangle { Triangle {} }

    /**
       Returns all coordinates that the shape occupies.
    Assume that you start at coordinate x: 0, y: 0.
    */
    fn area(&self, size: u32) -> Vec<Coordinate> {
        let size = size as i16;
        (0..size).fold(vec![], |mut acc, x| {
            acc.append(&mut plot(
                Coordinate::new(size / 2, 0),
                Coordinate::new(x, size),
            ));
            acc
        })
    }
}
