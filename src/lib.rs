#![cfg_attr(feature = "cargo-clippy", allow(clippy::derive_hash_xor_eq))]

extern crate gif;
extern crate image;
extern crate pythagoras;
extern crate rand;

/// Collection of helper macros.
#[macro_use]
pub mod macros;

/// Converts data in to Nodes and Groups.
pub mod data;

/// Extra functionality for groups.
pub mod group;

/// Extra functionality for nodes.
pub mod node;

/// Extra functionality for coordinates.
pub mod coordinate;

/// Functions to manage large numbers of placable entities.
pub mod map;

/// Shapes used to calculate areas.
pub mod shape;

/// Useful functions not bound to any specific functionality.
pub mod tools;

#[cfg(test)]
mod tests;

mod consts {
    pub const MAX_LINKS: usize = 5;
    pub const NETWORK_REM: usize = 666;
    pub const DEFAULT_SIZE: u16 = 4;
    pub const DEFAULT_SHADE: u16 = 20;
    pub const DEFAULT_LINK_SIZE: u16 = 2;
    pub const DEFAULT_RGBA: image::Rgba<u8> = image::Rgba {
        data: [0, 0, 0, 255],
    };
}

// ------------------------------------------------------------------

/// Holds a Coordinate on a x and y plane.
/// It's implemented in Nodes, Groups, HL and the Location trait to enable
/// differennt structures to be drawn.
#[derive(Debug, Eq, Copy, Clone, Default)]
pub struct Coordinate {
    pub x: i16,
    pub y: i16,
}

/// Connection between links. HL stands for HashLink, because it uses hashes for
/// references to other points.
#[derive(Copy, PartialEq, Eq, Clone, Debug, Default)]
pub struct HL {
    pub style: u8,
    pub f: u64,
    pub t: u64,
    pub from: Option<Coordinate>,
    pub to: Option<Coordinate>,
}

/// A Location object that can be drawn on an image, along with set size and
/// color.
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
/// It contains a Node used for Group meta data.
#[derive(Clone, Debug)]
pub struct Group {
    settings: Node,
    pub nodes: Vec<Node>,
}

/// High abstraction Map which helps position objects.
#[derive(Clone, Debug, Default)]
pub struct Map {
    image: Option<IW>,
    add: Coordinate,
}

/// Enables traversing through a network of connected nodes.
/// Checking if a path is valid and setting new paths.
#[derive(Clone, Copy)]
pub struct Network<T: Draw + Hash + std::marker::Copy> {
    pub hash_map: [Option<T>; consts::NETWORK_REM],
}

// ------------------------------------------------------------------

/// Provides the functions to create a generic shape.
pub trait Shape {
    fn new() -> Self;
    fn area(&self, size: u32) -> Vec<Coordinate>;
}

// ------------------------------------------------------------------

/// Provides the function to retrieve a hash from a structure.
pub trait Hash {
    fn hash(&self) -> u64;
}

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

/// Image wrapper around the Image crate to enable better debugging panics.
#[derive(Clone, Debug)]
pub struct IW {
    img: image::ImageBuffer<image::Rgba<u8>, Vec<u8>>,
}

impl IW {
    /// Retrieves the private image field.
    pub fn image(&self) -> &image::ImageBuffer<image::Rgba<u8>, Vec<u8>> { &self.img }

    /// Wraps around Image put_pixel but indicates failed positions.
    pub fn put<L: Location>(&mut self, l: &L, color: image::Rgba<u8>) {
        if cfg!(debug_assertions)
            && (l.x() as u32 > self.img.width() || l.y() as u32 >= self.img.height())
        {
            panic!(
                "({}) out of bound of image ({}, {})",
                l.position(),
                self.img.width(),
                self.img.height()
            );
        }
        self.put_unsafe(l, color);
    }

    pub fn put_unsafe<L: Location>(&mut self, l: &L, color: image::Rgba<u8>) {
        self.img.put_pixel(l.x() as u32, l.y() as u32, color);
    }

    pub fn dimensions(&self) -> Coordinate {
        coordinate!(self.img.width(), self.img.height())
    }
}

// ------------------------------------------------------------------

/// Makes it possible to find connected nodes in networks.
pub trait Find: Hash + Location {
    /// Matches the Hashes and returns Some if it matches.
    fn find<H: Hash>(&self, hash: H) -> Option<Coordinate> {
        if self.hash() == hash.hash() {
            return Some(self.position());
        }
        None
    }
}

impl Find for HL {}

impl Find for Node {}

impl Find for Group {
    /// Recursively calls find as the group contains sets of Nodes.
    fn find<H: Hash>(&self, hash: H) -> Option<Coordinate> {
        let f = tools::find(hash.hash(), &self.nodes);
        f.and_then(|x| Some(x.position()))
    }
}

// ------------------------------------------------------------------

/// Enables retrieving the minimum and maximum position for the structure.
pub trait MinMax {
    fn min_max(&self) -> (Coordinate, Coordinate);
}

impl MinMax for HL {
    fn min_max(&self) -> (Coordinate, Coordinate) {
        let zero = coordinate!();
        let to = self.to.unwrap_or(zero);
        (self.position(), to)
    }
}

impl MinMax for Node {
    fn min_max(&self) -> (Coordinate, Coordinate) {
        let mut max = coordinate!(consts::DEFAULT_SIZE);
        let mut min = self.position();
        min -= max;
        max += self.geo;
        (min, max)
    }
}

impl MinMax for Group {
    fn min_max(&self) -> (Coordinate, Coordinate) { group::parameters(self) }
}

// ------------------------------------------------------------------

/// Enables the structure to be located by X or Y.
pub trait Location {
    /// Retrieves the position Coordinates.
    fn position(&self) -> Coordinate;

    /// Returns if the positions are equal or not.
    fn eq<L: Location>(&self, other: &L) -> bool { self.position() == other.position() }

    /// Retrieves the X coordinate.
    fn x(&self) -> i16 { self.position().x }

    /// Retrieves the Y coordinate.
    fn y(&self) -> i16 { self.position().y }

    /// Returns the sum of the x and y value.
    fn sum(&self) -> i16 { self.x() + self.y() }
}

impl Location for HL {
    fn position(&self) -> Coordinate {
        let zero = coordinate!();
        self.from.unwrap_or(zero)
    }
}

impl Location for Node {
    fn position(&self) -> Coordinate { self.geo.position() }
}

impl Location for Group {
    fn position(&self) -> Coordinate { self.settings.position() }
}

impl Location for Coordinate {
    fn position(&self) -> Coordinate { *self }
}

// ------------------------------------------------------------------

/// Functions required to draw the structure on the image.
pub trait Draw {
    fn draw<S: Shape>(&self, image: IW, offset: Coordinate, shape: &S) -> IW;
    fn size(&self) -> u32;
    fn links(&self) -> &[HL];
}

impl Draw for Node {
    fn draw<S: Shape>(&self, mut image: IW, offset: Coordinate, shape: &S) -> IW {
        let s = consts::DEFAULT_LINK_SIZE / 2;
        let pos = self.geo + offset - coordinate!(s, s);

        for link in &self.links {
            image = link.draw(image, offset, u32::from(consts::DEFAULT_LINK_SIZE));
        }

        for o in shape.area(self.size()) {
            let color = if o.x == 0 || o.y == 0 {
                let c = self
                    .color
                    .data
                    .iter()
                    .map(|x| x.saturating_add(consts::DEFAULT_SHADE as u8))
                    .collect::<Vec<_>>();
                image::Rgba([c[0], c[1], c[2], c[3]])
            } else {
                self.color
            };
            let c = pos + o;
            image.put_unsafe(&c, color);
        }
        image
    }

    fn size(&self) -> u32 {
        self.radius
            .unwrap_or_else(|| u32::from(consts::DEFAULT_SIZE))
    }

    fn links(&self) -> &[HL] { &self.links }
}

impl Draw for Group {
    /// Draws the Nodes inside that Group. If none the Group is draw as blank.
    fn draw<S: Shape>(&self, mut image: IW, mut offset: Coordinate, shape: &S) -> IW {
        offset += self.position();
        for node in &self.nodes {
            image = node.draw(image, offset, shape);
        }
        image
    }

    fn size(&self) -> u32 {
        let mut max = 0;
        for node in &self.nodes {
            max = std::cmp::max(max, node.size());
        }
        max + self
            .settings
            .radius
            .unwrap_or_else(|| u32::from(consts::DEFAULT_SIZE))
    }

    fn links(&self) -> &[HL] { &self.settings.links() }
}

// ------------------------------------------------------------------

impl From<Coordinate> for Node {
    fn from(c: Coordinate) -> Self {
        let mut node = node!(c);
        node.hash = (c.x + c.y) as u64;
        node
    }
}

impl From<Group> for Node {
    fn from(group: Group) -> Self { group.settings }
}

impl From<Coordinate> for Group {
    fn from(c: Coordinate) -> Self {
        let mut group = Group::new_simple(c.x, c.y);
        group.set().hash = (c.x + c.y) as u64;
        group
    }
}

impl From<Node> for Group {
    fn from(node: Node) -> Self {
        let mut group = Group::new_simple(node.x(), node.y());
        group.settings = node;
        group
    }
}

impl From<Node> for Coordinate {
    fn from(node: Node) -> Self { node.position() }
}

impl From<Group> for Coordinate {
    fn from(group: Group) -> Self { group.position() }
}

// ------------------------------------------------------------------

impl std::ops::Add for Coordinate {
    type Output = Coordinate;

    fn add(self, other: Coordinate) -> Coordinate {
        Coordinate::new(self.x + other.x, self.y + other.y)
    }
}

impl std::ops::Sub for Coordinate {
    type Output = Coordinate;

    fn sub(self, other: Coordinate) -> Coordinate {
        Coordinate::new(self.x - other.x, self.y - other.y)
    }
}

impl std::ops::AddAssign for Coordinate {
    fn add_assign(&mut self, other: Coordinate) {
        self.x += other.x;
        self.y += other.y;
    }
}

impl std::ops::SubAssign for Coordinate {
    fn sub_assign(&mut self, other: Coordinate) {
        self.x -= other.x;
        self.y -= other.y;
    }
}

// ------------------------------------------------------------------

impl Coordinate {
    /// Constructs a Coordinate struct.
    pub fn new(x: i16, y: i16) -> Self { Coordinate { x, y } }

    /// Returns true if either x or y is less than the input.
    /// ```
    /// # use pathfinder::Coordinate;
    /// let c = Coordinate::new(10, 10);
    /// assert!(c.lt(11));
    /// ```
    pub fn lt(self, lt: i16) -> bool { self.x < lt || self.y < lt }

    /// Returns the absolute/positive equivilent.
    /// ```
    /// # use pathfinder::Coordinate;
    /// let c = Coordinate::new(-10, 10);
    /// assert_eq!(c.abs(), Coordinate::new(10, 10));
    /// ```
    pub fn abs(self) -> Coordinate { Coordinate::new(self.x.abs(), self.y.abs()) }

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

    /// Gets the center position of the node accounting for size.
    pub fn center(&self) -> Coordinate {
        let half = coordinate!(self.size() / 2, self.size() / 2);
        self.position() + half
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
        tools::find(other.hash, self.links()).is_some()
    }

    /// Links a list of nodes together in the order they are indexed.
    /// A list of A, B, C. Will result in them being linked as: A -> B -> C.
    ///
    /// ```
    /// # use pathfinder::Node;
    /// let nodes = Node::from_list(&[(0, 0), (20, 20)]);
    /// let linked_list = Node::linked_list(nodes);
    /// ```
    pub fn linked_list(list: Vec<Node>) -> Vec<Self> {
        Node::linked_list_predicate(list, &|_, _| true)
    }

    /// Returns a specific link if it exists. Returns none if not.
    ///
    /// ```
    /// # #[macro_use] extern crate pathfinder;
    /// # use pathfinder::{Coordinate, Node};
    /// # fn main() {
    /// let node = node!();
    /// let hl = node.hl(0);
    /// assert!(hl.is_err());
    /// # }
    /// ```
    ///
    /// ```
    /// # #[macro_use] extern crate pathfinder;
    /// # use pathfinder::{Coordinate, Node};
    /// # fn main() {
    /// let a = node!("A", 0, 0);
    /// let mut b = node!("B", 50, 50);
    /// b.link(&a);
    /// assert!(b.hl(0).is_ok());
    /// # }
    /// ```
    pub fn hl(&self, index: usize) -> std::io::Result<&HL> {
        if index > self.get_link_avail_index() || !self.links[index].is_connected() {
            Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                "index too large",
            ))
        } else {
            Ok(&self.links[index])
        }
    }

    /// See Node::hl for examples. Returns a mutable variant.
    pub fn hl_mut(&mut self, index: usize) -> std::io::Result<&mut HL> {
        if index > self.get_link_avail_index() || !self.links[index].is_connected() {
            Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "index {} too large or not connected. Index: {} expected. Connection status: \
                     {}",
                    index,
                    self.get_link_avail_index(),
                    self.links[index].is_connected()
                ),
            ))
        } else {
            Ok(&mut self.links[index])
        }
    }

    /// Links a list of nodes together in the order they are indexed.
    /// A list of A, B, C. Will result in them being linked as: A -> B -> C.
    /// ```
    /// use pathfinder::Node;
    /// let nodes = Node::from_list(&[(0, 0), (20, 20)]);
    /// let linked_list = Node::linked_list(nodes);
    /// ```
    pub fn linked_list_predicate(
        mut list: Vec<Node>,
        f: &Fn(Coordinate, Coordinate) -> bool,
    ) -> Vec<Self> {
        let mut prev = coordinate!();
        let mut prev_h = 0;
        for node in &mut list {
            if prev_h != 0 && f(prev, node.geo) {
                let mut link = HL::new(node.hash, prev_h);
                link.to = Some(prev);
                link.from = Some(node.geo);
                let i = node.get_link_avail_index();
                node.links[i] = link;
            }

            prev_h = node.hash();
            prev = node.geo;
        }
        list
    }

    /// Returns the next point which is available to link.
    /// ```
    /// # #[macro_use] extern crate pathfinder;
    /// # use pathfinder::{Coordinate, Node};
    /// # fn main() {
    /// let a = node!("A", 0, 0);
    /// let mut b = node!("B", 50, 50);
    /// assert_eq!(b.get_link_avail_index(), 0);
    /// b.link(&a);
    /// assert_eq!(b.get_link_avail_index(), 1);
    /// # }
    /// ```
    pub fn get_link_avail_index(&self) -> usize {
        self.links()
            .iter()
            .position(|x| !x.is_connected())
            .or(Some(consts::MAX_LINKS - 1))
            .unwrap()
    }

    /// Removes all connects leaving this node. This still leaves connections
    /// going towards this node.
    /// ```
    /// # #[macro_use] extern crate pathfinder;
    /// # use pathfinder::{Coordinate, Node};
    /// # fn main() {
    /// let a = node!(0, 0);
    /// let mut b = node!(50, 50);
    /// b.link(&a);
    /// assert!(b.hl(0).is_ok());
    /// b.disconnect();
    /// assert!(b.hl(0).is_err());
    /// # }
    /// ```
    pub fn disconnect(&mut self) { self.links = [HL::new(0, 0); consts::MAX_LINKS]; }

    /// Links Node self to another point that has Hash and Location implemented.
    /// ```
    /// # #[macro_use] extern crate pathfinder;
    /// # use pathfinder::{Coordinate, Location, Node};
    /// # fn main() {
    /// let b: Node = node!("B", 100, 100);
    /// let mut a: Node = node!("A", 0, 0);
    /// a.link(&b);
    /// assert!(a.is_directly_connected(&b));
    /// # }
    /// ```
    pub fn link<P: Hash + Location>(&mut self, other: &P) {
        let i = self.get_link_avail_index();
        self.links[i] = HL {
            style: 0,
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
            style: 0,
            f,
            t,
            from: None,
            to: None,
        }
    }

    /// Style setter. Values are:
    /// 0: Bresenhem.
    /// 1: Straight.
    /// 2: Ellipse.
    pub fn style(&mut self, s: u8) { self.style = s; }

    /// Checks if the HL has two endpoint hashes.
    pub fn is_connected(&self) -> bool { self.f != 0 && self.t != 0 }

    /// Draws the HL on an Image Wrapper.
    fn draw(&self, mut image: IW, mut offset: Coordinate, size: u32) -> IW {
        let (mut from, mut to) = self.min_max();
        if !self.is_connected() || from == to {
            return image;
        }
        let s = (size / 2) as i16;
        offset += coordinate!(s);
        from += offset;
        to += offset;

        for i in 0..size {
            for j in 0..size {
                let add = coordinate!(j, i) - coordinate!(s);
                let col = (size - i) as u8 * consts::DEFAULT_SHADE as u8;
                let plot = match self.style {
                    0 => tools::plot_type(from + add, to + add, &tools::plot_bresenham),
                    1 => tools::plot_type(from + add, to + add, &tools::plot_rectangle),
                    2 => tools::plot_type(from + add, to + add, &tools::plot_ellipse),
                    _ => tools::plot(from + add, to + add),
                };
                let _ = plot
                    .iter()
                    .map(|c| image.put_unsafe(c, image::Rgba([col, col, col, u8::max_value()])))
                    .collect::<Vec<_>>();
            }
        }
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

    /// Set the radius for the group's meta-data.
    pub fn radius(&mut self, radius: u32) { self.settings.radius = Some(radius); }

    /// Retrieves the nodes drawing in the group. Positions are relative to the
    /// group.
    pub fn nodes(&self) -> &Vec<Node> { &self.nodes }

    /// Retrieves a mutable group meta data.
    pub fn set(&mut self) -> &mut Node { &mut self.settings }

    /// Adds a set of nodes randomly located inside the group.
    pub fn add(&mut self, nr: u32) {
        for _ in 0..nr {
            let co = coordinate::gen_within_radius(self.position(), self.size());
            let mut node = node!(co);
            node.color = self.gen_color(co);
            self.push(node);
        }
    }

    /// Applies the closure over each mutable child node.
    pub fn each(&mut self, func: &Fn(&mut Node)) {
        for node in self.nodes.iter_mut() {
            func(node);
        }
    }

    /// Sets the color of the Group.
    pub fn color(&mut self, rgba: image::Rgba<u8>) { self.settings.color = rgba; }

    /// Plots node according to the fn provided.
    /// The closure parameter is the number of children the group has.
    pub fn node_plot(&mut self, calc: &Fn(usize) -> Coordinate) {
        let c = coordinate::calc(self.position(), self.nodes.len(), calc);
        let color = self.gen_color(c);
        let mut node = node!(c);
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
        node.geo -= self.position();
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
        coordinate::rotate_around_axis(coordinate!(), &mut self.nodes, rad);
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
    /// Creates a new map, no min_max are intially required and are
    /// generated automatically when calling Map::map.
    pub fn new() -> Self {
        Map {
            image: None,
            add: coordinate!(),
        }
    }

    /// Saves the image to disk. Can be absolute or relative path.
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
        self.image.unwrap().image().save(path)
    }

    /// Unwraps the image wrapper
    pub fn consume(self) -> IW { self.image.unwrap() }

    /// Maps any struct that has implemented Draw, on to an ImageBuffer.
    /// # Examples
    /// ```
    /// # use pathfinder::*;
    /// let nodes: Vec<Node> = Node::from_list(&[(0, 0), (100, 100)]);
    /// // Add content to vectors.
    /// let mut map = Map::new();
    /// map = map.map(&nodes);
    /// ```
    pub fn map<T: Draw + Location + Hash + MinMax>(self, element: &[T]) -> Self {
        self.map_filter(&element, &|_| true)
    }

    /// Maps the elements but with an added filter parameter to exclude
    /// elements.
    pub fn map_filter<T: Draw + Location + Hash + MinMax>(
        self,
        element: &[T],
        filter: &Fn(&T) -> bool,
    ) -> Self {
        self.map_params(&element, &filter, &shape::Square::new())
    }

    /// Maps the elements with a specified shape struct.
    pub fn map_shape<T: Draw + Location + Hash + MinMax, S: Shape>(
        self,
        element: &[T],
        shape: &S,
    ) -> Self {
        self.map_params(&element, &|_| true, shape)
    }

    /// Maps the elements without stabalizing the positions on the canvas.
    pub fn map_absolute<T: Draw + Location + Hash + MinMax>(mut self, element: &[T]) -> Self {
        if self.image.is_none() {
            let (image, _) = map::gen_map(&element);
            self.image = Some(IW { img: image });
        }
        self.map(element)
    }

    /// Maps the elements but with all added parameters.
    pub fn map_params<T: Draw + Location + Hash + MinMax, S: Shape>(
        mut self,
        element: &[T],
        filter: &Fn(&T) -> bool,
        shape: &S,
    ) -> Self {
        if self.image.is_none() {
            let (image, add) = map::gen_map(&element);
            self.image = Some(IW { img: image });
            self.add = add;
        }

        let add = self.add;
        self.image = Some(
            element
                .iter()
                .filter(|x| filter(x))
                .fold(self.image.unwrap(), |img, x| x.draw(img, add, shape)),
        );
        self
    }
}

impl Network<Node> {
    /// Calculates the path from node A to node B.
    /// # Examples
    /// ```
    /// use pathfinder::{Coordinate, Network, Node};
    /// let nodes = Node::from_list(&[(0, 0), (10, 10), (20, 20), (30, 30)]);
    /// let mut nodes = Node::linked_list(nodes);
    /// let path = Network::new(nodes).path("A", "D").unwrap();
    /// assert_eq!(path.len(), 4);
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
