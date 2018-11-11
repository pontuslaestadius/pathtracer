#![cfg_attr(feature = "cargo-clippy", allow(clippy::derive_hash_xor_eq))]

extern crate gif;
extern crate image;
extern crate pythagoras;
extern crate rand;

pub mod coordinate;
pub mod data;
pub mod group;
pub mod map;
pub mod node;
pub mod shape;
pub mod tools;

mod tests;

/// Holds a position used for Nodes and Groups.
#[derive(Debug, Eq, Copy, Clone, Default)]
pub struct Coordinate {
    pub x: i16,
    pub y: i16,
}

#[derive(Copy, PartialEq, Eq, Clone, Debug, Default)]
pub struct HL {
    pub f: u64,
    pub t: u64,
    pub from: Option<Coordinate>,
    pub to: Option<Coordinate>,
}

/// A positioned object that can be drawn on an image::ImageBuffer.
#[derive(Copy, Clone, Debug)]
pub struct Node {
    pub hash: u64,
    pub geo: Coordinate,
    pub color: image::Rgba<u8>,
    pub radius: Option<u32>,
    links: [HL; 10],
}

/// Holds a set of nodes and applies properties to all child nodes when drawn.
/// The group itself has no displayed output and is not visible.
#[derive(Clone, Debug)]
pub struct Group {
    pub settings: Node,
    pub nodes: Vec<Node>,
}

#[derive(Clone, Debug, Default)]
pub struct Map {
    pub image: Option<image::ImageBuffer<image::Rgba<u8>, Vec<u8>>>,
    pub add: (i16, i16),
    pub size: u32,
}

#[derive(Clone, Copy)]
pub struct Network<T: Draw + Hash + std::marker::Copy> {
    pub hash_map: [Option<T>; 666],
}

// ------------------------------------------------------------------

pub trait Shape {
    fn new() -> Self;
    fn area(&self, size: u32) -> Vec<Coordinate>;
}

pub trait Hash {
    fn get_hash(&self) -> u64;
}

// ------------------------------------------------------------------

pub trait Location {
    fn get_coordinate(&self) -> Coordinate;
    fn find(&self, hash: u64) -> Option<Coordinate>;
    fn get_parameters(&self) -> (Coordinate, Coordinate);
}

impl Location for HL {
    fn get_coordinate(&self) -> Coordinate { self.from.unwrap_or(Coordinate::new(0, 0)) }

    fn get_parameters(&self) -> (Coordinate, Coordinate) {
        let to = self.to.unwrap_or(Coordinate::new(0, 0));
        (self.get_coordinate(), to)
    }

    fn find(&self, hash: u64) -> Option<Coordinate> {
        if self.t == hash {
            return Some(self.get_coordinate());
        }
        None
    }
}

impl Location for Node {
    fn get_coordinate(&self) -> Coordinate { self.geo }

    fn get_parameters(&self) -> (Coordinate, Coordinate) { (self.geo, self.geo) }

    fn find(&self, hash: u64) -> Option<Coordinate> {
        if self.hash == hash {
            return Some(self.geo);
        }
        None
    }
}

impl Location for Group {
    fn get_coordinate(&self) -> Coordinate { self.settings.get_coordinate() }

    fn get_parameters(&self) -> (Coordinate, Coordinate) { group::get_parameters(self) }

    fn find(&self, hash: u64) -> Option<Coordinate> {
        let f = tools::find(hash, &self.nodes);
        f.and_then(|x| Some(x.get_coordinate()))
    }
}

// ------------------------------------------------------------------

pub trait Draw {
    fn draw<S: Shape>(
        &self,
        image: image::ImageBuffer<image::Rgba<u8>, Vec<u8>>,
        x_offset: i16,
        y_offset: i16,
        size: u32,
        shape: &S,
    ) -> image::ImageBuffer<image::Rgba<u8>, Vec<u8>>;
    fn get_size(&self) -> u32;
    fn get_links(&self) -> &[HL];
}

impl Draw for Node {
    fn draw<S: Shape>(
        &self,
        mut image: image::ImageBuffer<image::Rgba<u8>, Vec<u8>>,
        x_offset: i16,
        y_offset: i16,
        size: u32,
        shape: &S,
    ) -> image::ImageBuffer<image::Rgba<u8>, Vec<u8>> {
        let x = self.geo.x + x_offset as i16;
        let y = self.geo.y + y_offset as i16;

        for link in &self.links {
            image = link.draw(image, x_offset, y_offset, size);
        }

        for offset in shape.area(size) {
            image.put_pixel((x + offset.x) as u32, (y + offset.y) as u32, self.color);
        }
        image
    }

    fn get_size(&self) -> u32 { self.radius.unwrap_or(4) }

    fn get_links(&self) -> &[HL] { &self.links }
}

impl Draw for Group {
    /// Draws the Nodes inside that Group. If none the Group is draw as blank.
    fn draw<S: Shape>(
        &self,
        mut image: image::ImageBuffer<image::Rgba<u8>, Vec<u8>>,
        x_offset: i16,
        y_offset: i16,
        size: u32,
        shape: &S,
    ) -> image::ImageBuffer<image::Rgba<u8>, Vec<u8>> {
        image = self.settings.draw(image, x_offset, y_offset, size, shape);
        for node in &self.nodes {
            image = node.draw(image, x_offset, y_offset, size, shape);
        }
        image
    }

    // Returns the largest node that exists within the group.
    fn get_size(&self) -> u32 {
        let mut max = 0;
        for node in &self.nodes {
            max = std::cmp::max(max, node.get_size());
        }
        match self.settings.radius {
            Some(e) => max + e / 10,
            None => max,
        }
    }

    fn get_links(&self) -> &[HL] { &self.settings.links }
}

// ------------------------------------------------------------------

impl Hash for HL {
    fn get_hash(&self) -> u64 { self.t }
}

impl Hash for Node {
    fn get_hash(&self) -> u64 { self.hash }
}

impl Hash for Group {
    fn get_hash(&self) -> u64 { self.settings.get_hash() }
}

// ------------------------------------------------------------------

impl Coordinate {
    /// Constructs a Coordinate struct.
    pub fn new(x: i16, y: i16) -> Self { Coordinate { x, y } }

    /// Calculates the different in x and y of two Coordinates.
    pub fn diff(self, other: Coordinate) -> (i16, i16) { coordinate::diff(self, other) }

    /// Creates a list of coordinates from a list of tuples with x and y
    /// positions.
    pub fn from_list(list: &[(i16, i16)]) -> Vec<Coordinate> {
        coordinate::from_list(&list, &|c, _i| c)
    }
}

impl Node {
    /// Constructs a Node struct.
    pub fn new(name: &str, geo: Coordinate) -> Self {
        Node {
            hash: data::calculate_hash(&name),
            geo,
            color: image::Rgba {
                data: [0, 0, 0, 255],
            },
            radius: None,
            links: [HL::new(0, 0); 10],
        }
    }

    /// Converts a list of tuples (x,y) to a Vector of Nodes.
    /// Names are assigned from "A" and upwards automatically.
    ///
    /// ```
    /// use pathfinder::Node;
    /// let list = [(0, 0), (10, 10), (15, 15)];
    /// let nodes = Node::from_list(&list);
    /// assert_eq!(nodes.len(), 3);
    /// ```
    pub fn from_list(list: &[(i16, i16)]) -> Vec<Self> {
        coordinate::from_list(&list, &|c, i| {
            Node::new(&std::char::from_u32(65 + i as u32).unwrap().to_string(), c)
        })
    }

    /// Looks through all connected Nodes and returns if they are connected.
    pub fn is_directly_connected(&self, other: &Node) -> bool {
        tools::find(other.hash, self.get_links()).is_some()
    }

    /// Links a list of nodes together in the order they are indexed.
    /// A list of A, B, C. Will result in them being linked as: A -> B -> C.
    ///
    /// ```
    /// use pathfinder::Node;
    /// let nodes = Node::from_list(&[(0, 0), (20, 20)]);
    /// let linked_list = Node::linked_list(nodes);
    /// ```
    pub fn linked_list(mut list: Vec<Node>) -> Vec<Self> {
        let mut prev = Coordinate::new(0, 0);
        let mut prev_h = 0;
        for node in &mut list {
            if prev_h != 0 {
                let mut link = HL::new(node.hash, prev_h);
                link.to = Some(prev);
                link.from = Some(node.geo);
                node.links[0] = link;
            }

            prev_h = node.hash;
            prev = node.geo;
        }
        list
    }

    /// Links Node self to the provided node's coordinate.
    ///
    /// ```
    /// use pathfinder::{Coordinate, Location, Node};
    /// let b: Node = Node::new("B", Coordinate::new(100, 100));
    /// let mut a: Node = Node::new("A", Coordinate::new(0, 0));
    /// a.link(&b);
    /// assert!(a.is_directly_connected(&b));
    /// ```
    pub fn link(&mut self, other: &Node) {
        for link in &mut self.links {
            if link.t == 0 {
                link.f = self.hash;
                link.t = other.hash;
                link.from = Some(self.geo);
                link.to = Some(other.geo);
                return;
            }
        }
    }
}

impl HL {
    pub fn new(f: u64, t: u64) -> Self {
        HL {
            f,
            t,
            from: None,
            to: None,
        }
    }

    fn draw(
        &self,
        mut image: image::ImageBuffer<image::Rgba<u8>, Vec<u8>>,
        x_offset: i16,
        y_offset: i16,
        size: u32,
    ) -> image::ImageBuffer<image::Rgba<u8>, Vec<u8>> {
        if self.f == 0 || self.t == 0 {
            return image;
        }
        let (from, to) = self.get_parameters();
        if from == to {
            return image;
        }

        let x_offset = x_offset + (size / 2) as i16;
        let y_offset = y_offset + (size / 2) as i16;

        let _ = tools::plot(
            Coordinate::new(from.x + x_offset, from.y + y_offset),
            Coordinate::new(to.x + x_offset, to.y + y_offset),
        )
        .iter()
        .map(|c| {
            image.put_pixel(
                c.x as u32,
                c.y as u32,
                image::Rgba {
                    data: [0, 0, 0, 255],
                },
            )
        })
        .collect::<Vec<_>>();
        image
    }
}

impl Group {
    /// Constructs a new Group
    pub fn new(name: &str, coordinates: Coordinate) -> Self {
        Group {
            settings: Node::new(name, coordinates),
            nodes: Vec::new(),
        }
    }

    /// Adds a Node dynamically to the Group.
    pub fn new_node(&mut self) { group::add_node(self, None, None, None); }

    /// Sets the color of the Group.
    pub fn set_color<T: Into<u8>>(&mut self, r: T, g: T, b: T) {
        self.settings.color = image::Rgba {
            data: [r.into(), g.into(), b.into(), 255],
        }
    }

    /// Plots node according to the fn provided.
    pub fn node_plot(&mut self, calc: &Fn(usize) -> Coordinate) {
        let c = coordinate::calc(self.get_coordinate(), self.nodes.len(), calc);
        let color = self.gen_color(c);
        let mut node = Node::new("", c);
        node.color = color;
        self.nodes.push(node);
    }

    /// Adds a Node with a specific minimum and maximum distance from the
    /// center of the Group.
    pub fn new_node_min_max(&mut self, min: u32, max: u32) {
        group::add_node(self, None, Some(min), Some(max));
    }

    /// Removes all non-essentials from the standard implementation.
    pub fn new_simple(x: i16, y: i16) -> Self {
        Group::new(&(x + y).to_string(), Coordinate::new(x, y))
    }

    /// Pushes a Node to the Group.
    pub fn push(&mut self, node: Node) { self.nodes.push(node); }

    /// Returns a dynamic radius based on the number of Nodes in the Group.
    pub fn get_dynamic_radius(&self) -> u32 {
        match self.settings.radius {
            Some(x) => x,
            None => 7 + self.nodes.len() as u32 / 2,
        }
    }

    /// Generates an image::Rgba based on the color of the Group and the
    /// distance from center.
    pub fn gen_color(&self, coordinates: Coordinate) -> image::Rgba<u8> {
        tools::range_color(
            self.get_dynamic_radius() as i16,
            self.settings.color,
            self.settings.geo,
            coordinates,
        )
    }

    /// Converts a list of tuples (x,y) to a Vector of Groups.
    /// Names are assigned from "A" and upwards automatically.
    ///
    /// ```
    /// use pathfinder::Group;
    /// let list = [(0, 0), (10, 10), (15, 15)];
    /// let groups = Group::from_list(&list);
    /// assert_eq!(groups.len(), 3);
    /// ```
    pub fn from_list(list: &[(i16, i16)]) -> Vec<Self> {
        coordinate::from_list(&list, &|c, i| {
            Group::new(&std::char::from_u32(65 + i as u32).unwrap().to_string(), c)
        })
    }

    /// Links together two groups.
    /// ```
    /// use pathfinder::{Coordinate, Group, Location};
    /// let b: Group = Group::new("B", Coordinate::new(100, 100));
    /// let mut a: Group = Group::new("A", Coordinate::new(0, 0));
    /// a.link(&b);
    /// ```
    pub fn link(&mut self, other: &Group) { self.settings.link(&other.settings); }
}

impl<T: Draw + Hash + std::marker::Copy> Network<T> {
    pub fn new(mut elements: Vec<T>) -> Self {
        let mut hash_map: [Option<T>; 666] = [None; 666];
        while !elements.is_empty() {
            let e = elements.remove(0);
            hash_map[(e.get_hash() % 666) as usize] = Some(e);
        }

        Network { hash_map }
    }
}

// ------------------------------------------------------------------

impl Map {
    /// Creates a new map, no parameters are intially required and are
    /// generated automatically when calling Map::map.
    pub fn new() -> Self {
        Map {
            image: None,
            add: (0, 0),
            size: 4,
        }
    }

    /// Saves the image to disk. Can be absolute or relative path.
    ///
    /// # Examples
    /// ```
    /// # use pathfinder::*;
    /// # use std::path::Path;
    /// let nodes = Node::from_list(&[(0, 0), (10, 10)]);
    /// Map::new()
    ///     .map(&nodes)
    ///     .save(Path::new("/tmp/example.png"))
    ///     .unwrap();
    /// ```
    pub fn save(self, path: &std::path::Path) -> Result<(), std::io::Error> {
        self.image.unwrap().save(path)
    }

    /// Maps any struct that has implemented Draw, on to an ImageBuffer.
    ///
    /// # Examples
    /// ```
    /// # use pathfinder::*;
    /// let nodes: Vec<Node> = vec![
    ///     Node::new("1", Coordinate::new(0, 0)),
    ///     Node::new("2", Coordinate::new(100, 100)),
    /// ];
    /// // Add content to vectors.
    /// let mut map = Map::new();
    /// map = map.map(&nodes);
    /// ```
    pub fn map<T: Draw + Location + Hash>(mut self, element: &[T]) -> Self {
        if self.image.is_none() {
            let (image, add) = map::gen_map(&element);
            self.image = Some(image);
            self.add = add;
        }

        let sq = shape::Square::new();
        for e in element {
            self.image = Some(e.draw(self.image.unwrap(), self.add.0, self.add.1, self.size, &sq));
        }
        self
    }

    /// Maps the elements without stabalizing the positions on the canvas.
    pub fn map_absolute<T: Draw + Location + Hash>(mut self, element: &[T]) -> Self {
        if self.image.is_none() {
            let (image, _) = map::gen_map(&element);
            self.image = Some(image);
        }
        self.map(element)
    }
}

impl Network<Node> {
    /// Calculates the path from node A to node B.
    ///
    /// # Examples
    /// ```
    /// use pathfinder::{Coordinate, Network, Node};
    /// let b = Node::new("B", Coordinate::new(20, 20));
    /// let mut a = Node::new("A", Coordinate::new(0, 0));
    /// a.link(&b);
    /// let path = Network::new(vec![a, b]).path("A", "B");
    /// assert_eq!(path, vec!(b, a));
    /// ```
    /// ```
    /// use pathfinder::{Coordinate, Network, Node};
    /// let nodes = Node::from_list(&[(0, 0), (10, 10), (20, 20), (30, 30)]);
    /// let mut nodes = Node::linked_list(nodes);
    /// let path = Network::new(nodes).path("D", "A");
    /// assert_eq!(path.len(), 4);
    /// for i in 0..path.len() {
    ///     assert_eq!(path[i].geo.x, i as i16 * 10);
    /// }
    /// ```
    pub fn path<'a>(&'a self, a: &str, b: &str) -> Vec<Node> {
        map::network::path(self, a, b, &map::network::path_shortest_leg)
    }

    /// Returns if the given hash exists in the network.
    ///
    /// # Examples
    /// ```
    /// use pathfinder::{Coordinate, Network, Node};
    /// let nodes = Node::from_list(&[(0, 0), (10, 10), (20, 20), (30, 30), (40, 40)]);
    /// let network = Network::new(nodes.clone());
    /// assert!(network.get("A").is_some());
    /// assert!(network.get("E").is_some());
    /// assert!(network.get("F").is_none());
    /// ```
    pub fn get(&self, element: &str) -> Option<Node> { map::network::get(self, element) }
}
