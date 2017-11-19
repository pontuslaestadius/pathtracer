use std::io;
use std::io::prelude::*;
use std::str::FromStr;
use std::fs::{OpenOptions, File};


use super::coordinates::Coordinates;
use super::super::tools::{constants, util};
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
        let mut move_x = self.from.x -self.to.x;
        let move_y = self.from.y -self.to.y;

        let pixel: Rgba<u8> = Rgba {data: [0,0,0,255]};

        let mut x = self.from.x +x_offset;
        let mut y = self.from.y +y_offset;

        while move_x != 0 {
            image.put_pixel(x as u32, y as u32, pixel);
            if move_x <0 {
                x += 1;
                move_x += 1;
            } else {
                x -= 1;
                move_x -= 1;
            }
        }

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
