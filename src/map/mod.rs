extern crate image;

use super::*;
use image::Rgba;
use std::cmp;

pub mod gif;
pub mod network;

pub fn gen_map<T: Location + Draw>(
    list: &[T],
) -> (image::ImageBuffer<Rgba<u8>, Vec<u8>>, (i16, i16)) {
    let min_max = min_max(&list);
    let res = gen_map_dimensions(min_max);
    (gen_canvas(res.0, res.1), gen_stuff(min_max))
}

/// Returns the difference between the lowest and highest x and y values,
/// respectively.
fn gen_map_dimensions(min_max: ((i16, i16), (i16, i16))) -> (u32, u32) {
    let x = min_max.0;
    let y = min_max.1;
    ((x.1 - x.0) as u32, (y.1 - y.0) as u32)
}

/// Finds the min and max of a list and returns ((minX, minY),(maxX, maxY)).
fn min_max<T: Location + Draw>(list: &[T]) -> ((i16, i16), (i16, i16)) {
    let mut size: i16 = 4;
    let mut min = Coordinate::new(0, 0);
    let mut max = Coordinate::new(0, 0);

    for item in list {
        size = cmp::max(size, item.size() as i16);
        let (imin, imax) = item.parameters();

        max.x = cmp::max(max.x, imax.x);
        min.x = cmp::min(min.x, imin.x);
        max.y = cmp::max(max.y, imax.y);
        min.y = cmp::min(min.y, imin.y);
    }

    // We add a safety border using size.
    ((min.x - size, max.x + size), (min.y - size, max.y + size))
}

/// Sets the additions required to center the pixels on the map.
/// This allows for negative placements of Coordinates, when adjusted with this
/// function.
fn gen_stuff(min_max: ((i16, i16), (i16, i16))) -> (i16, i16) { (-(min_max.0).0, -(min_max.1).0) }

/// Generates a canvas from the image crate.
fn gen_canvas(w: u32, h: u32) -> image::ImageBuffer<Rgba<u8>, Vec<u8>> {
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
        let nodes = Node::from_list(&[(-50, 50), (50, -50), (0, 25), (25, 0)]);
        let res = min_max(&nodes);
        assert_eq!(res, ((-54, 54), (-54, 54)));
    }
}
