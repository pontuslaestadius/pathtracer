use super::coordinates::Coordinates;
use image::{ImageBuffer, Rgba};

use super::util::{gen_rgba, plot};

/*
     Link
     --------
     Holds connections between two structures with coordinates.
 */

pub struct Link<'a> {
    pub from: &'a Coordinates,
    pub to: &'a Coordinates,
}

impl<'a> Link<'a> {

    pub fn draw(&self, image: &mut ImageBuffer<Rgba<u8>, Vec<u8>>, x_offset: i16, y_offset: i16, size: u32) {

        //let pixel: Rgba<u8> = Rgba {data: [0,0,0,255]};
        let pixel = gen_rgba(); // TODO this is not reliable since it's random.

        let mut a = Coordinates::new(
            self.from.x +x_offset,
            self.from.y +y_offset
        );

        let mut b = Coordinates::new(
            self.to.x +x_offset,
            self.to.y +y_offset
        );

        plot(&a, &b).iter().map(|c|
            image.put_pixel( c.x  as u32, c.y as u32, pixel)
        ).collect::<Vec<_>>();
    }

    /// Creates a new Link and binds two nodes together.
    pub fn new(from: &'a Coordinates, to: &'a Coordinates) -> Link<'a> {
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
