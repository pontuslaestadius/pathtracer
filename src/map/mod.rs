extern crate image;

use image::Rgba;
use super::*;
use std::cmp;

pub mod network;
pub mod gif;

/// Returns the difference between the lowest and highest x and y values, respectively.
pub fn gen_map_dimensions(min_max: ((i16, i16), (i16, i16))) -> (u32, u32) {
    let x = min_max.0;
    let y = min_max.1;
    ((x.1 - x.0) as u32, (y.1 - y.0) as u32)
}

/// Finds the min and max of a list and returns ((minX, minY),(maxX, maxY)).
pub fn min_max<T: Location + Draw>(list: &[T]) -> ((i16, i16), (i16, i16)) {
    let mut size: i16 = 4;

    let mut min_x: i16 = 0;
    let mut min_y: i16 = 0;
    let mut max_x: i16 = 0;
    let mut max_y: i16 = 0;

    for item in list {
        size = cmp::max(size, item.get_size() as i16);

        let (min,max) = item.get_parameters();

        max_x = cmp::max(max_x, max.x);
        min_x = cmp::min(min_x, min.x);
        max_y = cmp::max(max_y, max.y);
        min_y = cmp::min(min_y, min.y);
    }

    // We add a safety border using size.
    ((min_x -size, max_x +size), (min_y -size, max_y +size))
}

/// Sets the additions required to center the pixels on the map.
/// This allows for negative placements of Coordinates, when adjusted with this function.
pub fn gen_stuff(min_max: ((i16, i16), (i16, i16))) -> (i16, i16) {
    (-(min_max.0).0, -(min_max.1).0)
}

/// Generates a canvas from the image crate.
pub fn gen_canvas(w: u32, h: u32) -> image::ImageBuffer<Rgba<u8>, Vec<u8>> {
    image::DynamicImage::new_rgba8(w, h).to_rgba()
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_gen_canvas() {
        let image = gen_canvas(50, 50);
        assert_eq!(image.width(), 50);
        assert_eq!(image.height(), 50);
    }

    #[test]
    fn test_gen_stuff() {
        let res = gen_stuff(((-10, -11), (12, 13)));
        assert_eq!(res, (10, -12));
    }

    #[test]
    fn test_gen_map_dimensions() {
        let res = gen_map_dimensions(((-50, -45), (0, 30)));
        assert_eq!(res, (5, 30));
    }

    #[test]
    fn test_min_max() {
        let nodes = Node::from_list(&[
            (-50, 50), (50, -50), 
            (0, 25), (25, 0) 
            ]);
        let res = min_max(&nodes);
        assert_eq!(res, ((-54, 54), (-54, 54)));
    }
}


