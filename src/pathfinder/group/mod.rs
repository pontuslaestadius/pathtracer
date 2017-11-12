
use pathfinder::node::{Node, coordinates};
use image::{ImageBuffer, Rgba};

pub struct Group {
    pub name: String,
    pub nodes: Vec<Node>,
    pub geo: coordinates::Coordinates,
}
impl Group {
    pub fn new(name: String, coordinates: coordinates::Coordinates) -> Group {
        Group {
            name,
            nodes: Vec::new(),
            geo: coordinates,
        }
    }

    pub fn draw(&self, image: &mut ImageBuffer<Rgba<u8>, Vec<u8>>, x_offset: u32, y_offset: u32, size: u32) {
        for node in self.nodes.iter() {
            node.draw(image, x_offset, y_offset, size);
        }
    }
}

pub fn min_max(list: &[Group]) -> ((i16, i16), (i16, i16)) {
    // Finds the min and max nodes, for scaling and boundaries.
    let mut min_x = 0;
    let mut min_y = 0;
    let mut max_x = 0;
    let mut max_y = 0;

    for group in list {
        for node in group.nodes.iter() {
            // Iterates over the nodes and finds the minimum and maximum x and y values.
            if node.geo.x > max_x {
                max_x = node.geo.x;
            }
            if min_x > node.geo.x {
                min_x = node.geo.x;
            }

            if node.geo.y > max_y {
                max_y = node.geo.y;
            }
            if min_y > node.geo.y {
                min_y = node.geo.y;
            }
        }
    }

    ((min_x, max_x), (min_y, max_y))
}