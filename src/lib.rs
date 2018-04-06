#![cfg_attr(feature="clippy", feature(plugin))]
#![cfg_attr(feature="clippy", plugin(clippy))]

extern crate image;
extern crate rand;

pub mod node;
pub mod map;
pub mod tools;
pub mod group;
pub mod data;

mod tests;

/// Holds a position used for Nodes and Groups.
#[derive(Debug, Eq, Copy, Clone)]
pub struct Coordinate {
    pub x: i16,
    pub y: i16,
}

/// A positioned object that can be drawn on an image::ImageBuffer.
#[derive(Clone)]
pub struct Node {
    pub hash: u64,
    pub geo: Coordinate,
    pub color: image::Rgba<u8>,
    pub radius: Option<u32>,
}

/// Holds a set of nodes and applies properties to all child nodes when drawn.
/// The group itself has no displayed output and is not visible.
#[derive(Clone)]
pub struct Group {
    pub settings: Node,
    pub nodes: Vec<Node>,
}

impl Coordinate {
    /// Constructs a Coordinate struct.
    pub fn new(x: i16, y: i16) -> Coordinate {
        Coordinate {
            x,
            y
        }
    }
}

impl Node {
    /// Constructs a Node struct.
    pub fn new(name: &str, geo: Coordinate) -> Node {
        Node {
            hash: data::calculate_hash(&name),
            geo,
            color: image::Rgba {data: [0,0,0,255]},
            radius: None,
        }
    }
}

impl Group {
    /// Constructs a new Group
    pub fn new(name: &str, coordinates: Coordinate) -> Group {
        Group {
            settings: Node::new(name, coordinates),
            nodes: Vec::new(),
        }
    }
}

// ------------------------------------------------------------------

impl Coordinate {
    // Calculates the different in x and y of two Coordinates.
    pub fn diff(&self, other: &Coordinate) -> (i16, i16) {
        node::coordinates::diff(&self, other)
    }
}

impl Node {
    /// Draws a node on an ImageBuffer.
    pub fn draw(&self, image: &mut image::ImageBuffer<image::Rgba<u8>, Vec<u8>>, x_offset: u32, y_offset: u32, size: u32) {
        // Adds the offset to the geo location as i16. Because geo can be negative but offset can not.
        let x = (self.geo.x +x_offset as i16) as u32;
        let y = (self.geo.y +y_offset as i16) as u32;
        let size = match self.radius {
            Some(_) => self.radius.unwrap(),
            None => size
        };
        for i in 0..size {
            for j in 0..size {
                image.put_pixel(x +i, y +j,    self.color);
                image.put_pixel(x +i +1, y +j +1, image::Rgba {data: [0,0,0,255]});
            }
        }
    }
}

impl Group {

    /// Returns the nodes that exists inside the Group.
    pub fn get_nodes(&self) -> &Vec<Node> {
        &self.nodes
    }

    /// Draws the Nodes inside that Group. If none the Group is draw as blank.
    pub fn draw(&self, image: &mut image::ImageBuffer<image::Rgba<u8>, Vec<u8>>, x_offset: u32, y_offset: u32, size: u32) {
        for node in self.nodes.iter() {
            node.draw(image, x_offset, y_offset, size);
        }
    }

    /// Adds a Node dynamically to the Group.
    pub fn new_node(&mut self, name: &str) {
        let geo = node::coordinates::gen_radius(&self.settings.geo, 0, self.get_dynamic_radius());
        self.new_node_inner(geo, name);
    }

    /// Adds a Node with a static distance from the center of the Group.
    pub fn new_node_min_auto(&mut self, name: &str, min: u32) -> &Node {
        let geo = node::coordinates::gen_radius(&self.settings.geo, 0, min+5);
        self.new_node_inner(geo, name)
    }

    /// Adds a Node with a specific minimum and maximum distance from the center of the Group.
    pub fn new_node_min_max(&mut self, name: &str, min: u32, max: u32) -> &Node {
        let geo = node::coordinates::gen_radius(&self.settings.geo, min, max);
        self.new_node_inner(geo, name)
    }

    /// Constructs a new node for the Group and mirrors the properties to it.
    pub fn new_node_inner(&mut self, geo: Coordinate, name: &str) -> &Node {
        let mut node = Node::new(name,geo.clone());
        node.color = self.gen_color(geo);
        self.push(node);
        &self.nodes.get(self.nodes.len() -1).unwrap()
    }

    /// Pushes a Node to the Group.
    pub fn push(&mut self, node: Node) {
        self.nodes.push(node);
    }

    /// Returns a dynamic radius based on the number of Nodes in the Group.
    pub fn get_dynamic_radius(&self) -> u32 {
        match self.settings.radius {
            Some(x) => x,
            None => 10 + self.nodes.len()as u32 /2,
        }
    }

    // Generates an image::Rgba based on the color of the Group and the distance from center.
    pub fn gen_color(&self, coordinates: Coordinate) -> image::Rgba<u8> {
        let radius = self.get_dynamic_radius() as i16;
        let (x_dif, y_dif) = self.settings.geo.diff(&coordinates);
        let x_scale: f64 = (x_dif as f64/radius as f64) as f64;
        let y_scale: f64 = (y_dif as f64/radius as f64) as f64;
        let c = self.settings.color.data;
        let max_multi: f64 = ((c[0] as i32 + c[1] as i32 + c[2] as i32)/3) as f64;
        let modify = (-max_multi*(x_scale+y_scale)/2.0) as i32;
        image::Rgba {data: [
            tools::border(c[0], modify),
            tools::border(c[1], modify),
            tools::border(c[2], modify),
            tools::border(c[3], 0)
        ]}
    }
}
