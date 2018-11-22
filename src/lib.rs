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

mod consts {
    pub const MAX_LINKS: usize = 5;
    pub const NETWORK_REM: usize = 666;
    pub const DEFAULT_SIZE: u16 = 4;
    pub const DEFAULT_RGBA: image::Rgba<u8> = image::Rgba {
        data: [0, 0, 0, 255],
    };
}

// ------------------------------------------------------------------

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
    links: [HL; consts::MAX_LINKS],
}

/// Holds a set of nodes and applies properties to all child nodes when drawn.
/// The group itself has no displayed output and is not visible.
#[derive(Clone, Debug)]
pub struct Group {
    settings: Node,
    nodes: Vec<Node>,
}

#[derive(Clone, Debug, Default)]
pub struct Map {
    pub image: Option<image::ImageBuffer<image::Rgba<u8>, Vec<u8>>>,
    pub add: (i16, i16),
    pub size: u16,
}

#[derive(Clone, Copy)]
pub struct Network<T: Draw + Hash + std::marker::Copy> {
    pub hash_map: [Option<T>; consts::NETWORK_REM],
}

// ------------------------------------------------------------------

pub trait Shape {
    fn new() -> Self;
    fn area(&self, size: u32) -> Vec<Coordinate>;
}

pub trait Hash {
    fn hash(&self) -> u64;
}

// ------------------------------------------------------------------

impl std::fmt::Display for Coordinate {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}, {}", self.x, self.y)
    }
}

impl std::fmt::Display for HL {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{} -> {}", self.f, self.t)
    }
}

// ------------------------------------------------------------------

pub trait Location {
    fn position(&self) -> Coordinate;
    fn find(&self, hash: u64) -> Option<Coordinate>;
    fn parameters(&self) -> (Coordinate, Coordinate);
}

impl Location for HL {
    fn position(&self) -> Coordinate {
        let zero = Coordinate::new(0, 0);
        self.from.unwrap_or(zero)
    }

    fn parameters(&self) -> (Coordinate, Coordinate) {
        let zero = Coordinate::new(0, 0);
        let to = self.to.unwrap_or(zero);
        (self.position(), to)
    }

    fn find(&self, hash: u64) -> Option<Coordinate> {
        if self.t == hash {
            return Some(self.position());
        }
        None
    }
}

impl Location for Node {
    fn position(&self) -> Coordinate { self.geo }

    fn parameters(&self) -> (Coordinate, Coordinate) { (self.geo, self.geo) }

    fn find(&self, hash: u64) -> Option<Coordinate> {
        if self.hash == hash {
            return Some(self.geo);
        }
        None
    }
}

impl Location for Group {
    fn position(&self) -> Coordinate { self.settings.position() }

    fn parameters(&self) -> (Coordinate, Coordinate) { group::parameters(self) }

    fn find(&self, hash: u64) -> Option<Coordinate> {
        let f = tools::find(hash, &self.nodes);
        f.and_then(|x| Some(x.position()))
    }
}

// ------------------------------------------------------------------

pub trait Draw {
    fn draw<S: Shape>(
        &self,
        image: image::ImageBuffer<image::Rgba<u8>, Vec<u8>>,
        offset: Coordinate,
        size: u32,
        shape: &S,
    ) -> image::ImageBuffer<image::Rgba<u8>, Vec<u8>>;
    fn size(&self) -> u32;
    fn links(&self) -> &[HL];
}

impl Draw for Node {
    fn draw<S: Shape>(
        &self,
        mut image: image::ImageBuffer<image::Rgba<u8>, Vec<u8>>,
        offset: Coordinate,
        size: u32,
        shape: &S,
    ) -> image::ImageBuffer<image::Rgba<u8>, Vec<u8>> {
        let mut pos = self.geo;
        pos.add(offset);

        for link in &self.links {
            image = link.draw(image, offset, size);
        }

        for o in shape.area(size) {
            image.put_pixel((pos.x + o.x) as u32, (pos.y + o.y) as u32, self.color);
        }
        image
    }

    fn size(&self) -> u32 { self.radius.unwrap_or(4) }

    fn links(&self) -> &[HL] { &self.links }
}

impl Draw for Group {
    /// Draws the Nodes inside that Group. If none the Group is draw as blank.
    fn draw<S: Shape>(
        &self,
        mut image: image::ImageBuffer<image::Rgba<u8>, Vec<u8>>,
        mut offset: Coordinate,
        size: u32,
        shape: &S,
    ) -> image::ImageBuffer<image::Rgba<u8>, Vec<u8>> {
        image = self.settings.draw(image, offset, size, shape);
        offset.add(self.position());
        for node in &self.nodes {
            image = node.draw(image, offset, size, shape);
        }
        image
    }

    /// Returns the largest node that exists within the group.
    fn size(&self) -> u32 {
        let mut max = 0;
        for node in &self.nodes {
            max = std::cmp::max(max, node.size());
        }
        match self.settings.radius {
            Some(e) => max + e / 10,
            None => max,
        }
    }

    fn links(&self) -> &[HL] { &self.settings.links() }
}

// ------------------------------------------------------------------

impl Hash for HL {
    fn hash(&self) -> u64 { self.t }
}

impl Hash for Node {
    fn hash(&self) -> u64 { self.hash }
}

impl Hash for Group {
    fn hash(&self) -> u64 { self.settings.hash() }
}

// ------------------------------------------------------------------

impl Coordinate {
    /// Constructs a Coordinate struct.
    pub fn new(x: i16, y: i16) -> Self { Coordinate { x, y } }

    /// Calculates the different in x and y of two Coordinates.
    pub fn diff(self, other: Coordinate) -> (i16, i16) { coordinate::diff(self, other) }

    /// Adds the other to self.
    pub fn add(&mut self, other: Coordinate) {
        self.x += other.x;
        self.y += other.y;
    }

    /// Subtracts the other from self.
    pub fn subtract(&mut self, other: Coordinate) {
        self.x -= other.x;
        self.y -= other.y;
    }

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
            color: consts::DEFAULT_RGBA,
            radius: None,
            links: [HL::new(0, 0); consts::MAX_LINKS],
        }
    }

    /// Retrive coordinate from a csv format.
    pub fn from_file(path: &str) -> Result<Vec<Self>, std::io::Error> { node::from_file(path) }

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
        tools::find(other.hash, self.links()).is_some()
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
                let i = node.get_link_avail_index();
                node.links[i] = link;
            }

            prev_h = node.hash;
            prev = node.geo;
        }
        list
    }

    /// Returns the next point which is available to link.
    fn get_link_avail_index(&self) -> usize {
        for (i, link) in self.links().iter().enumerate() {
            if !link.is_connected() {
                return i;
            }
        }
        consts::MAX_LINKS - 1
    }

    /// Links Node self to another point that has Hash and Location implemented.
    ///
    /// ```
    /// use pathfinder::{Coordinate, Location, Node};
    /// let b: Node = Node::new("B", Coordinate::new(100, 100));
    /// let mut a: Node = Node::new("A", Coordinate::new(0, 0));
    /// a.link(&b);
    /// assert!(a.is_directly_connected(&b));
    /// ```
    pub fn link<P: Hash + Location>(&mut self, other: &P) {
        let i = self.get_link_avail_index();
        self.links[i] = HL {
            f: self.hash,
            t: other.hash(),
            from: Some(self.geo),
            to: Some(other.position()),
        };
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

    pub fn is_connected(&self) -> bool { self.f != 0 && self.t != 0 }

    fn draw(
        &self,
        mut image: image::ImageBuffer<image::Rgba<u8>, Vec<u8>>,
        mut offset: Coordinate,
        size: u32,
    ) -> image::ImageBuffer<image::Rgba<u8>, Vec<u8>> {
        let (mut from, mut to) = self.parameters();
        if self.f == 0 || self.t == 0 || from == to {
            return image;
        }
        let s = (size / 2) as i16;
        offset.add(Coordinate::new(s, s));
        from.add(offset);
        to.add(offset);

        let _ = tools::plot(from, to)
            .iter()
            .map(|c| image.put_pixel(c.x as u32, c.y as u32, consts::DEFAULT_RGBA))
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

    pub fn radius(&mut self, radius: u32) { self.settings.radius = Some(radius); }

    pub fn nodes(&self) -> &Vec<Node> { &self.nodes }

    pub fn node(&self) -> &Node { &self.settings }

    /// Sets the color of the Group.
    pub fn color(&mut self, rgba: image::Rgba<u8>) { self.settings.color = rgba; }

    /// Plots node according to the fn provided.
    /// The provided function value is the number of children the group has.
    pub fn node_plot(&mut self, calc: &Fn(usize) -> Coordinate) {
        let c = coordinate::calc(self.position(), self.nodes.len(), calc);
        let color = self.gen_color(c);
        let mut node = Node::new("", c);
        node.color = color;
        self.push(node);
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
    pub fn push(&mut self, mut node: Node) {
        node.geo.subtract(self.position());
        self.nodes.push(node);
    }

    /// Returns a dynamic radius based on the number of Nodes in the Group.
    pub fn dynamic_radius(&self) -> u32 {
        match self.settings.radius {
            Some(x) => x,
            None => u32::from(consts::DEFAULT_SIZE) + self.nodes.len() as u32 / 2,
        }
    }

    /// Rotates all the nodes inside the group.
    pub fn rotate(&mut self, rad: f64) {
        // Use 0, 0 because the self.nodes positions relative.
        coordinate::rotate_around_axis(Coordinate::new(0, 0), &mut self.nodes, rad);
    }

    /// Generates an image::Rgba based on the color of the Group and the
    /// distance from center.
    pub fn gen_color(&self, coordinates: Coordinate) -> image::Rgba<u8> {
        tools::range_color(
            self.dynamic_radius() as i16,
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
        let mut hash_map: [Option<T>; consts::NETWORK_REM] = [None; consts::NETWORK_REM];
        while !elements.is_empty() {
            let e = elements.remove(0);
            hash_map[(e.hash() as usize % consts::NETWORK_REM)] = Some(e);
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
            size: consts::DEFAULT_SIZE,
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
            self.image = Some(e.draw(
                self.image.unwrap(),
                Coordinate::new(self.add.0, self.add.1),
                self.size.into(),
                &sq,
            ));
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
    pub fn path<'a>(&'a self, a: &str, b: &str) -> std::io::Result<Vec<Node>> {
        let mut path = map::network::path(self, b, a, &map::network::path_shortest_leg)?;
        path.reverse();
        Ok(path)
    }

    /// Mimics path behaviour but works in reverse, Meaning stepping back in
    /// the links.
    pub fn path_rev<'a>(&'a self, a: &str, b: &str) -> std::io::Result<Vec<Node>> {
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
