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

/* FIXME Stuck on iterative mut assignment.
/// Returns a list of Links connecting the Nodes in the order they were provided.
/// E.g. provided a list with three nodes the result would be:
/// 1----2----3, dashes representing links.
pub fn sequentially_link_nodes<'a, T: Shape + Draw>(nodes: &'a mut [Node<'a, T>]) {
    let mut b: Option<&mut Node<T>> = None;

    for (i, node) in nodes.iter_mut().enumerate() {
        let mut a = node;

        //let tmp = b;
        //b = Some(a);

        if b.is_none() {
            continue;
        }

        a.link(&b.unwrap());

    }
}
*/

/// Finds the min and max Nodes and returns ((minX, minY),(maxX, maxY))
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
    let x = min_max.0;
    let y = min_max.1;
    (-x.0, -y.0)
}

/// Generates a canvas from the image crate.
pub fn gen_canvas(w: u32, h: u32) -> image::ImageBuffer<Rgba<u8>, Vec<u8>> {
    image::DynamicImage::new_rgba8(w, h).to_rgba()
}

