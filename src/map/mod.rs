extern crate image;

use super::*;
use image::Rgba;
use std::cmp;

/// Gif wrapper for managing frames with pathfinder
pub mod gif;

/// Connecting Nodes logic
pub mod network;

/**
Returns the underlaying image used for the Map struct.
*/
pub fn gen_map<T: Location + Draw + MinMax>(
    list: &[T],
) -> (image::ImageBuffer<Rgba<u8>, Vec<u8>>, (Coordinate)) {
    let (min, max) = min_max(&list);
    let diff = max - min;
    let add = Coordinate::new(-min.x, -min.y);
    let image = gen_canvas(diff.x as u32, diff.y as u32);
    (image, add)
}

/**
Finds the min and max of a list and returns (min, max).

the min and max use the size of the Draw trait to enlarge the are the min, max occupy.
*/
fn min_max<T: Location + Draw + MinMax>(list: &[T]) -> (Coordinate, Coordinate) {
    let mut size: i16 = consts::DEFAULT_SIZE as i16;
    let mut min = coordinate!();
    let mut max = coordinate!();

    for item in list {
        size = cmp::max(size, item.size() as i16);
        let (imin, imax) = item.min_max();

        max.x = cmp::max(max.x, imax.x);
        min.x = cmp::min(min.x, imin.x);
        max.y = cmp::max(max.y, imax.y);
        min.y = cmp::min(min.y, imin.y);
    }

    let size = coordinate!(size / 4);
    (min - size, max + size)
}

/**
Generates a canvas from the image crate.
*/
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
    fn test_min_max() {
        let nodes = Node::from_list(&[(-50, 50), (50, -50), (0, 25), (25, 0)]);
        let (min, max) = min_max(&nodes);
        assert_eq!(min, Coordinate::new(-55, -55));
        assert_eq!(max, Coordinate::new(55, 55));
    }
}
