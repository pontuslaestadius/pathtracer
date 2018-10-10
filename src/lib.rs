#![cfg_attr(feature = "clippy", feature(plugin))]
#![cfg_attr(feature = "clippy", plugin(clippy))]

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
pub struct HashLink {
    pub from_hash: u64,
    pub to_hash: u64,
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
    links: [HashLink; 10],
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

impl Location for HashLink {
    fn get_coordinate(&self) -> Coordinate {
        if self.from.is_none() {
            Coordinate::new(0, 0)
        } else {
            self.from.unwrap()
        }
    }

    fn get_parameters(&self) -> (Coordinate, Coordinate) {
        let to = if self.to.is_none() {
            Coordinate::new(0, 0)
        } else {
            self.to.unwrap()
        };

        (self.get_coordinate(), to)
    }

    fn find(&self, hash: u64) -> Option<Coordinate> {
        if self.to_hash == hash {
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
        if f.is_some() {
            Some(f.unwrap().get_coordinate())
        } else {
            None
        }
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
    fn get_links(&self) -> &[HashLink];
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
            let xo = (x + offset.x) as u32;
            let yo = (y + offset.y) as u32;
            image.put_pixel(xo, yo, self.color);
        }
        image
    }

    fn get_size(&self) -> u32 {
        if self.radius.is_none() {
            4
        } else {
            self.radius.unwrap()
        }
    }

    fn get_links(&self) -> &[HashLink] { &self.links }
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

    fn get_links(&self) -> &[HashLink] { &self.settings.links }
}

// ------------------------------------------------------------------

impl Hash for HashLink {
    fn get_hash(&self) -> u64 { self.to_hash }
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
    pub fn new(x: i16, y: i16) -> Coordinate { Coordinate { x, y } }

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
    pub fn new(name: &str, geo: Coordinate) -> Node {
        Node {
            hash: data::calculate_hash(&name),
            geo,
            color: image::Rgba {
                data: [0, 0, 0, 255],
            },
            radius: None,
            links: [HashLink::new(0, 0); 10],
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
    pub fn from_list(list: &[(i16, i16)]) -> Vec<Node> {
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
    pub fn linked_list(mut list: Vec<Node>) -> Vec<Node> {
        let mut prev = Coordinate::new(0, 0);
        let mut prev_h = 0;
        for node in &mut list {
            if prev_h != 0 {
                let mut link = HashLink::new(node.hash, prev_h);
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
    /// assert_eq!(a.is_directly_connected(&b), true);
    /// ```
    pub fn link(&mut self, other: &Node) {
        for link in &mut self.links {
            if link.to_hash == 0 {
                link.from_hash = self.hash;
                link.to_hash = other.hash;
                link.from = Some(self.geo);
                link.to = Some(other.geo);
                return;
            }
        }
    }
}

impl HashLink {
    pub fn new(from_hash: u64, to_hash: u64) -> HashLink {
        HashLink {
            from_hash,
            to_hash,
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
        if self.from_hash == 0 || self.to_hash == 0 {
            return image;
        }
        let (from, to) = self.get_parameters();
        if from == to {
            return image;
        }

        let x_offset = x_offset + (size / 2) as i16;
        let y_offset = y_offset + (size / 2) as i16;

        let a = Coordinate::new(from.x + x_offset, from.y + y_offset);
        let b = Coordinate::new(to.x + x_offset, to.y + y_offset);

        let _ = tools::plot(a, b)
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
    pub fn new(name: &str, coordinates: Coordinate) -> Group {
        Group {
            settings: Node::new(name, coordinates),
            nodes: Vec::new(),
        }
    }

    /// Adds a Node dynamically to the Group.
    pub fn new_node(&mut self) { group::add_node(self, None, None, None); }

    /// Adds a Node with a specific minimum and maximum distance from the
    /// center of the Group.
    pub fn new_node_min_max(&mut self, min: u32, max: u32) {
        group::add_node(self, None, Some(min), Some(max));
    }

    /// Removes all non-essentials from the standard implementation.
    pub fn new_simple(x: i16, y: i16) -> Group { Group::new("", Coordinate::new(x, y)) }

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
    pub fn from_list(list: &[(i16, i16)]) -> Vec<Group> {
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
    pub fn new(mut elements: Vec<T>) -> Network<T> {
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
    pub fn new() -> Map {
        Map {
            image: None,
            add: (0, 0),
            size: 4,
        }
    }

    /// Saves the image to disk.
    pub fn save(self, path: &std::path::Path) -> Result<(), std::io::Error> {
        self.image.unwrap().save(path)
    }

    /// Maps any struct that has implemented Draw, on to an ImageBuffer.
    ///
    /// ```
    /// use pathfinder::*;
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

    // Maps the elements without stabalizing the positions on the canvas.
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
    /// ```
    /// use pathfinder::{map::network, Coordinate, Network, Node};
    /// let b = Node::new("B", Coordinate::new(20, 20));
    /// let mut a = Node::new("A", Coordinate::new(0, 0));
    /// a.link(&b);
    /// let network = Network::new(vec![a, b]);
    /// let path = network.path("A", "B", &network::path_shortest_leg);
    /// assert_eq!(path, vec!(a, b));
    /// ```
    pub fn path<'a>(
        &'a self,
        a: &str,
        b: &str,
        algorithm: &Fn(&Network<Node>, &str, &str) -> Vec<Node>,
    ) -> Vec<Node> {
        map::network::path(self, a, b, algorithm)
    }

    /// Returns if the given hash exists in the network.
    pub fn get(&self, element: &str) -> Option<Node> { map::network::get(self, element) }
}
