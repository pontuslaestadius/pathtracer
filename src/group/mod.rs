use super::data::calculate_hash;
use node::{Node, coordinates};
use image::{ImageBuffer, Rgba};
use tools::util::border;

pub struct Group {
    pub hash: u64,
    pub nodes: Vec<Node>,
    pub geo: coordinates::Coordinate,
    pub color: Rgba<u8>,
    pub radius: u32,
}
impl Group {
    pub fn new(name: &str, coordinates: coordinates::Coordinate, color: Rgba<u8>, radius: u32) -> Group {
        Group {
            hash: calculate_hash(&name),
            nodes: Vec::new(),
            geo: coordinates,
            color,
            radius
        }
    }

    pub fn draw(&self, image: &mut ImageBuffer<Rgba<u8>, Vec<u8>>, x_offset: u32, y_offset: u32, size: u32) {
        for node in self.nodes.iter() {
            node.draw(image, x_offset, y_offset, size);
        }
    }

    pub fn new_node(&mut self, name: String) {
        let geo = coordinates::gen_radius(&self.geo, 0, self.radius);
        self.new_node_inner(geo, name);
    }

    pub fn new_node_min_auto(&mut self, name: String, min: u32) -> &Node {
        let geo = coordinates::gen_radius(&self.geo, 0, min+5);
        self.new_node_inner(geo, name)
    }

    pub fn new_node_min_max(&mut self, name: String, min: u32, max: u32) -> &Node {
        let geo = coordinates::gen_radius(&self.geo, min, max);
        self.new_node_inner(geo, name)
    }

    pub fn new_node_inner(&mut self, geo: coordinates::Coordinate, name: String) -> &Node {

        let color = self.gen_color(geo.clone());

        self.push(
            Node {
                name,
                geo,
                color
            }
        );
        &self.nodes.get(self.nodes.len() -1).unwrap()
    }

    pub fn push(&mut self, node: Node) {
        self.nodes.push(node);
    }

    pub fn gen_color(&self, coordinates: coordinates::Coordinate) -> Rgba<u8> {

        let radius = self.radius as i16;

        let (x_dif, y_dif) = coordinates::diff(&self.geo, &coordinates);

        let x_scale: f64 = (x_dif as f64/radius as f64) as f64;
        let y_scale: f64 = (y_dif as f64/radius as f64) as f64;

        let c = self.color.data;
        let max_multi: f64 = ((c[0] as i32 + c[1] as i32 + c[2] as i32)/3) as f64;

        let modify = (-max_multi*(x_scale+y_scale)/2.0) as i32;

        Rgba {data: [
            border(c[0], modify),
            border(c[1], modify),
            border(c[2], modify),
            border(c[3], 0)
        ]}
    }
}

// Counts the amount of groups and child nodes.
pub fn count(list: &[Group]) -> (usize, usize) {
    let mut n: usize = 0;
    for g in list.iter() {
        n+=g.nodes.len();
    }
    (list.len(), n)
}

// Finds the min and max nodes, for scaling and boundaries.
pub fn min_max(list: &[Group]) -> ((i16, i16), (i16, i16)) {
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