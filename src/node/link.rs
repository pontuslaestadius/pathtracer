use super::coordinates::Coordinate;
use image::{ImageBuffer, Rgba};

use super::util::{gen_rgba, plot};

/*
     Link
     --------
     Holds connections between two structures with coordinates.
 */

pub struct Link<'a> {
    pub from: &'a Coordinate,
    pub to: &'a Coordinate,
}

impl<'a> Link<'a> {

    pub fn draw(&self, image: &mut ImageBuffer<Rgba<u8>, Vec<u8>>, x_offset: i16, y_offset: i16) {

        //let pixel: Rgba<u8> = Rgba {data: [0,0,0,255]};
        let pixel = gen_rgba(); // TODO this is not reliable since it's random.

        let a = Coordinate::new(
            self.from.x +x_offset,
            self.from.y +y_offset
        );

        let b = Coordinate::new(
            self.to.x +x_offset,
            self.to.y +y_offset
        );

        plot(&a, &b).iter().map(|c|
            image.put_pixel( c.x  as u32, c.y as u32, pixel)
        ).collect::<Vec<_>>();
    }

    /// Creates a new Link and binds two nodes together.
    pub fn new(from: &'a Coordinate, to: &'a Coordinate) -> Link<'a> {
        Link {
            from,
            to,
        }
    }
}

impl<'a> PartialEq for Link<'a> {
    fn eq(&self, other: &Link) -> bool {
        (self.from == other.from) &&
            (self.to == other.to)
    }
}
