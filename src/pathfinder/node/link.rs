use super::coordinates::Coordinates;
use image::{ImageBuffer, Rgba};

/*
     Link
     --------
     Holds connections between two struct with coordinates.
 */

pub struct Link<'a> {
    pub from: &'a Coordinates,
    pub to: &'a Coordinates,
}

impl<'a> Link<'a> {

    pub fn draw(&self, image: &mut ImageBuffer<Rgba<u8>, Vec<u8>>, x_offset: i16, y_offset: i16, size: u32) {
        let mut move_x = self.from.x -self.to.x +size as i16;
        let mut move_y = self.from.y -self.to.y +size as i16;

        let pixel: Rgba<u8> = Rgba {data: [0,0,0,255]};

        let mut x = self.from.x +x_offset;
        let mut y = self.from.y +y_offset;

        let mut x_ = Vec::new();
        let mut y_ = Vec::new();

        while x != self.to.x {
            if move_x <0 {
                x += 1;
                move_x += 1;
            } else {
                x -= 1;
                move_x -= 1;
            }
            x_.push(x);
        }

        while y != self.to.y {
            if move_y <0 {
                y += 1;
                move_y += 1;
            } else {
                y -= 1;
                move_y -= 1;
            }
            y_.push(y);
        }

        x = x_.remove(0);
        y = y_.remove(0);

        let mut pos = Vec::new();

        while !x_.is_empty() && !y_.is_empty() {

            if x_.len() == y_.len() {
                x = x_.remove(0);
                y = y_.remove(0);
            } else if x_.len() > y_.len()  {
                x = x_.remove(0);
            } else  /*x_.len() < y_.len()*/{
                y = y_.remove(0);
            }

            pos.push(Coordinates::new(x, y));
        }

        pos.iter().map(|c| image.put_pixel(c.x as u32, c.y as u32, pixel)).collect::<Vec<_>>();

    }

    /// Creates a new nodeLink and binds two nodes together.
    pub fn new<'b>(from: &'b Coordinates, to: &'b Coordinates, ) -> Link<'b> {
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
