#![cfg_attr(feature = "cargo-clippy", allow(clippy::derive_hash_xor_eq))]

extern crate gif;
extern crate image;
extern crate pythagoras;
extern crate rand;

#[macro_use]
extern crate log;

#[macro_use]
pub mod macros;

pub mod consts;
pub mod coordinate;
pub mod data;
pub mod group;
pub mod map;
pub mod node;
pub mod tools;
pub mod traits;

pub use traits::*;

#[cfg(test)]
mod tests;

// ------------------------------------------------------------------

/*
Holds a Coordinate on a x and y plane.
It's implemented in Nodes, Groups, HL and the Location trait to enable
differennt structures to be drawn.
 */
#[derive(Debug, Eq, Copy, Clone, Default)]
pub struct Coordinate {
    pub x: i16,
    pub y: i16,
}

/*
Connection between links. HL stands for HashLink, because it uses hashes for
references to other points.

## Examples

Given two Nodes, A and B.

A links to B.

This means that B does not know that A is connected to it.


Deleting node B in this scenario would impact drawing and pathing.
But does not have any direct impact on the HL since it only stores a Hash reference to the node it is linked to.
 */
#[derive(Copy, PartialEq, Eq, Clone, Debug, Default)]
pub struct HL {
    pub style: EdgeStyle,
    pub f: u64,
    pub t: u64,
    pub from: Option<Coordinate>,
    pub to: Option<Coordinate>,
}

/**
A Location object that can be drawn on an image, along with set size and color.
 */
#[derive(Copy, Clone, Debug)]
pub struct Node {
    pub hash: u64,
    pub geo: Coordinate,
    pub color: image::Rgba<u8>,
    pub radius: Option<u32>,
    links: [HL; consts::MAX_LINKS],
}

/**
Holds a set of nodes and applies properties to all child nodes when drawn.

The group itself has no displayed output and is not visible.

It contains a Node used for Group meta data.
 */
#[derive(Clone, Debug)]
pub struct Group {
    settings: Node,
    pub nodes: Vec<Node>,
}

/**
High abstraction Map which helps position objects.
 */
#[derive(Clone, Debug, Default)]
pub struct Map {
    image: Option<IW>,
    add: Coordinate,
}

/**
Enables traversing through a network of connected nodes.


Checking if a path is valid and setting new paths.
 */
#[derive(Clone, Copy)]
pub struct Network<T: Draw + Hash + std::marker::Copy> {
    pub hash_map: [Option<T>; consts::NETWORK_REM],
}

// ------------------------------------------------------------------

/**
Style setter for Edges.
Decides what algorithm will be performed to generate the edge on the image.


## Straight

Sharp L shape


## Ellipse

Curves the connection


## Direct

Uses Brasehem's line algorithm to directly correct the nodes.

 */
#[derive(Copy, PartialEq, Eq, Clone, Debug)]
pub enum EdgeStyle {
    Direct,
    Ellipse,
    Straight,
}

/**
 Creates a shape of coordinate points.

 Sampled from a Area.
*/
#[derive(Debug, Clone)]
pub enum Shape {
    Circle,
    Square,
    Triangle,
}

// ------------------------------------------------------------------

impl Shape {
    /**
     Returns all coordinates that the shape occupies.

     Assumes that 0 0 is the top-left of the node.

     ## Circle

     Algorithm is derived from:
     https://en.wikipedia.org/wiki/Midpoint_circle_algorithm

    */
    pub fn area(&self, area: usize) -> Vec<Coordinate> {
        match *self {
            Shape::Circle => Shape::circle_area(area),
            Shape::Square => Shape::square_area(area),
            Shape::Triangle => Shape::triangle_area(area),
        }
    }

    fn circle_area(area: usize) -> Vec<Coordinate> {
        let mut vec = Vec::new();
        let mut pos = coordinate!((area - 1), 0);
        let mut err: i16 = 1 - (area << 1) as i16;
        let mut d = Coordinate::new(err, 1);

        let q_plot = |x1, y1, x2, y2| tools::plot(coordinate!(x1, y1), coordinate!(x2, y2));

        while pos.x >= pos.y {
            for i in [1, -1].iter() {
                vec.append(&mut q_plot(pos.x, i * pos.y, -pos.x, i * pos.y));
                vec.append(&mut q_plot(i * pos.y, -pos.x, i * pos.y, pos.x));
            }

            if err <= 0 {
                pos.y += 1;
                d.y += 2;
                err += d.y;
            } else {
                pos.x -= 1;
                d.x += 2;
                err += d.x;
            }
        }

        vec
    }

    fn square_area(area: usize) -> Vec<Coordinate> {
        (0..area).fold(vec![], |mut acc, x| {
            for i in 0..area {
                acc.push(coordinate!(x, i));
            }
            acc
        })
    }

    fn triangle_area(area: usize) -> Vec<Coordinate> {
        (0..area).fold(vec![], |mut acc, x| {
            acc.append(&mut tools::plot(
                coordinate!(area / 2, 0),
                coordinate!(x, area),
            ));
            acc
        })
    }
}

// ------------------------------------------------------------------

impl std::default::Default for EdgeStyle {
    fn default() -> Self {
        EdgeStyle::Direct
    }
}

// ------------------------------------------------------------------

/**
Provides the function to retrieve a hash from a structure.
 */
impl Hash for HL {
    fn hash(&self) -> u64 {
        self.t
    }
}

impl Hash for Node {
    fn hash(&self) -> u64 {
        self.hash
    }
}

impl Hash for Group {
    fn hash(&self) -> u64 {
        self.settings.hash()
    }
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

/**
Image wrapper around the Image crate to enable better debugging panics.
 */
#[derive(Clone, Debug)]
pub struct IW {
    img: image::ImageBuffer<image::Rgba<u8>, Vec<u8>>,
}

impl IW {
    /**
    Retrieves the private image field.
     */
    pub fn image(&self) -> &image::ImageBuffer<image::Rgba<u8>, Vec<u8>> {
        &self.img
    }

    /**
    Wraps around Image put_pixel but indicates failed positions.

    Set debug_assertions flag to panic for out of bounds positions with improved debugging messages.
     */
    pub fn put<L: Location>(&mut self, l: &L, color: image::Rgba<u8>) {
        self.img.put_pixel(l.x() as u32, l.y() as u32, color);
    }

    /**
    Returns a coordinate with the width and height of the image buffer.
     */
    pub fn dimensions(&self) -> Coordinate {
        coordinate!(self.img.width(), self.img.height())
    }
}

// ------------------------------------------------------------------

impl Find for HL {}

impl Find for Node {}

impl Find for Group {
    /**
    Recursively calls find as the group contains sets of Nodes.
     */
    fn find<H: Hash>(&self, hash: H) -> Option<Coordinate> {
        let f = tools::find(hash.hash(), &self.nodes);
        f.and_then(|x| Some(x.position()))
    }
}

// ------------------------------------------------------------------

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
    /**
    Returns the the largest and smallest x and y position found in the group.


    ## Examples

    ```
    # #[macro_use] use pathfinder::*;
    # fn main() {
    let mut group = Group::new_simple(0, 0);
    group.push(node!(100, 100));
    let (min, max) = group.min_max();
    assert_eq!(min.x, 0);
    assert_eq!(max.x, 104);
    # }
    ```
    */
    fn min_max(&self) -> (Coordinate, Coordinate) {
        let mut min = coordinate!(0, 0);
        let mut max = coordinate!(0, 0);
        for node in &self.nodes {
            let (min2, max2) = node.min_max();
            max.x = std::cmp::max(max.x, max2.x);
            min.x = std::cmp::min(min.x, min2.x);
            max.y = std::cmp::max(max.y, max2.y);
            min.y = std::cmp::min(min.y, min2.y);
        }
        (min + self.position(), max + self.position())
    }
}

// ------------------------------------------------------------------

impl Location for HL {
    fn position(&self) -> Coordinate {
        let zero = coordinate!();
        self.from.unwrap_or(zero)
    }
}

impl Location for Node {
    fn position(&self) -> Coordinate {
        self.geo.position()
    }
}

impl Location for Group {
    fn position(&self) -> Coordinate {
        self.settings.position()
    }
}

impl Location for Coordinate {
    fn position(&self) -> Coordinate {
        *self
    }
}

// ------------------------------------------------------------------

impl Draw for Node {
    /**
    Draws the node on an IW.

    It is recommended to not use this directory.
    But instead use the Map struct, which uses this trait implementation.
    */
    fn draw(&self, mut image: IW, offset: Coordinate, shape: &Shape) -> IW {
        let s = consts::DEFAULT_LINK_SIZE / 2;
        let pos = self.geo + offset - coordinate!(s, s);

        for link in &self.links {
            image = link.draw(image, offset, u32::from(consts::DEFAULT_LINK_SIZE));
        }

        for o in shape.area(self.size() as usize) {
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
            image.put(&(pos + o), color);
        }
        image
    }

    fn size(&self) -> u32 {
        self.radius
            .unwrap_or_else(|| u32::from(consts::DEFAULT_SIZE))
    }

    fn links(&self) -> &[HL] {
        &self.links
    }
}

impl Draw for Group {
    /**
    Draws the Nodes inside that Group.

    If none the Group is draw as blank.
     */
    fn draw(&self, image: IW, mut offset: Coordinate, shape: &Shape) -> IW {
        offset += self.position();
        self.nodes
            .iter()
            .fold(image, |acc, node| node.draw(acc, offset, shape))
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

    fn links(&self) -> &[HL] {
        &self.settings.links()
    }
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
    fn from(group: Group) -> Self {
        group.settings
    }
}

impl From<Coordinate> for Group {
    fn from(c: Coordinate) -> Self {
        let mut group = cluster!(c);
        group.set().hash = (c.x + c.y) as u64;
        group
    }
}

impl From<Node> for Group {
    fn from(node: Node) -> Self {
        let mut group = cluster!(node.position());
        group.settings = node;
        group
    }
}

impl From<Node> for Coordinate {
    fn from(node: Node) -> Self {
        node.position()
    }
}

impl From<Group> for Coordinate {
    fn from(group: Group) -> Self {
        group.position()
    }
}

// ------------------------------------------------------------------

impl Coordinate {
    /**
    Constructs a Coordinate struct.


    ## Examples

    ```
    # #[macro_use] use pathfinder::*;
    # fn main() {
    Coordinate::new(10, 10);
    # }
    ```

    Invocation can be done used the macro coordinate!

    ```
    # #[macro_use] use pathfinder::*;
    # fn main() {
    coordinate!(10);
    # }
    ```

    These are equal.

    ```
    # #[macro_use] use pathfinder::*;
    # fn main() {
    let a = coordinate!(10);
    let b = Coordinate::new(10, 10);
    assert_eq!(a, b);
    # }
    ```
     */
    pub fn new(x: i16, y: i16) -> Self {
        Coordinate { x, y }
    }

    /**
    Returns true if either x or y is less than the input.


    ## Examples

    ```
    # use pathfinder::Coordinate;
    let c = Coordinate::new(10, 10);
    assert!(c.lt(11));
    ```
     */
    pub fn lt(self, lt: i16) -> bool {
        self.x < lt || self.y < lt
    }

    /**
    Returns the absolute coordinate equivilent.


    ## Examples

    ```
    # use pathfinder::Coordinate;
    let c = Coordinate::new(-10, 10);
    assert_eq!(c.abs(), Coordinate::new(10, 10));
    ```
     */
    pub fn abs(self) -> Coordinate {
        Coordinate::new(self.x.abs(), self.y.abs())
    }

    /**
    Creates a list of coordinates from a list of tuples with x and y positions.
     */
    pub fn from_list(list: &[(i16, i16)]) -> Vec<Coordinate> {
        coordinate::from_list(&list, &|c, _i| c)
    }
}

impl Node {
    /**
    Constructs a Node struct.

    The name is converted from a &str to a hash.
     */
    pub fn new(name: &str, geo: Coordinate) -> Self {
        Node {
            hash: data::calculate_hash(&name),
            geo,
            color: consts::DEFAULT_RGBA,
            radius: None,
            links: [HL::new(0, 0); consts::MAX_LINKS],
        }
    }

    /**
    Retrive coordinate from a csv format.

    ## Examples

    Example file format:

    100,20

    40,60

    30,30


    Would be equivalent to the following.

    ```
    # use pathfinder::Node;
    let list = [(100, 20), (40, 60), (30, 30)];
    let nodes = Node::from_list(&list);
    assert_eq!(nodes.len(), 3);
    ```
     */
    pub fn from_file(path: &str) -> Result<Vec<Self>, std::io::Error> {
        node::from_file(path)
    }

    /**
    Gets the center position of the node accounting for size.
     */
    pub fn center(&self) -> Coordinate {
        let half = coordinate!(self.size() / 2, self.size() / 2);
        self.position() + half
    }

    /**
    Converts a list of tuples (x,y) to a Vector of Nodes.

    Names are assigned from "A" and upwards automatically.


    ## Examples

    Create three nodes from a list.

    ```
    # use pathfinder::Node;
    let list = [(0, 0), (10, 10), (15, 15)];
    let nodes = Node::from_list(&list);
    assert_eq!(nodes.len(), 3);
    ```

    Returns an empty array if given an empty list.

    ```
    # use pathfinder::Node;
    let nodes = Node::from_list(&[]);
    assert_eq!(nodes.len(), 0);
    ```
     */
    pub fn from_list(list: &[(i16, i16)]) -> Vec<Self> {
        coordinate::from_list(&list, &|c, i| {
            Node::new(&std::char::from_u32(65 + i as u32).unwrap().to_string(), c)
        })
    }

    /**
    Looks through all connected Nodes and returns if they are connected.
     */
    pub fn is_directly_connected<P: Hash + Location>(&self, other: &P) -> bool {
        tools::find(other.hash(), self.links()).is_some()
    }

    /**
    Links a list of nodes together in the order they are indexed.


    A list of A, B, C.
    Will result in them being linked as: A -> B -> C.


    ## Examples

    ```
    # use pathfinder::Node;
    let nodes = Node::from_list(&[(0, 0), (20, 20)]);
    let linked_list = Node::linked_list(nodes);
    ```
     */
    pub fn linked_list(list: Vec<Node>) -> Vec<Self> {
        Node::linked_list_predicate(list, &|_, _| true)
    }

    /**
    Returns a specific link if it exists. Returns none if not.


    ## Examples

    If the Node has not been linked.

    ```
    # #[macro_use] extern crate pathfinder;
    # use pathfinder::{Coordinate, Node};
    # fn main() {
    let node = node!();
    let hl = node.hl(0);
    assert!(hl.is_err());
    # }
    ```

    Linking Nodes makes us able to interface with Edges.

    ```
    # #[macro_use] extern crate pathfinder;
    # use pathfinder::{Coordinate, Node};
    # fn main() {
    let a = node!("A", 0, 0);
    let mut b = node!("B", 50, 50);
    b.link(&a);
    assert!(b.hl(0).is_ok());
    # }
    ```


    ## Errors


    If the index is larger than the available HL.

    The available HL can be retrieved through get_link_avail_index.

    You can only retrieve HL which are connected to other nodes.

     */
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

    /**
    Returns a mutable reference to a HL.


    ## Examples

    A mutable variant is required when setting Edge styles.

    ```
    # #[macro_use] extern crate pathfinder;
    # use pathfinder::{EdgeStyle, Coordinate, Node};
    # fn main() -> std::io::Result<()> {
    let mut a = node!("A", 0, 0);
    let mut b = node!("B", 50, 50);
    b.link(&a);
    b.hl_mut(0)?.style(EdgeStyle::Straight);

    a.link(&b);
    a.hl_mut(0)?.style(EdgeStyle::Ellipse);
    Ok(())
    # }
    ```


    ## Errors

    If the index is larger than the available HL.

    You can only retrieve HL which are connected to other nodes.

     */
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

    /**
    Links a list of nodes together in the order they are indexed.


    A list of A, B, C. Will result in them being linked as: A -> B -> C.


    ## Examples

    The two following are the same.

    ```
    # use pathfinder::Node;
    let nodes = Node::from_list(&[(0, 0), (20, 20)]);
    let linked_list = Node::linked_list(nodes);
    ```

    ```
    # use pathfinder::Node;
    let nodes = Node::from_list(&[(0, 0), (20, 20)]);
    let linked_list = Node::linked_list_predicate(nodes, &|_, _| true);
    ```

    Will link the items with the same x value.

    ```
    # use pathfinder::Node;
    # let nodes = Node::from_list(&[(0, 0), (20, 20)]);
    let predicate = &|c1, c2| c1 > c2;
    let linked_list = Node::linked_list_predicate(nodes, predicate);
    ```
     */
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

    /**
    Returns the next point which is available to link.

    It is a good practice to call this method before attempting to call hl or hl_mut to having to error handle.


    ## Examples

    Connects two nodes.

    ```
    # #[macro_use] extern crate pathfinder;
    # use pathfinder::{Coordinate, Node};
    # fn main() {
    let a = node!("A", 0, 0);
    let mut b = node!("B", 50, 50);
    assert_eq!(b.get_link_avail_index(), 0);
    b.link(&a);
    assert_eq!(b.get_link_avail_index(), 1);
    # }
    ```

    Disconnecting decreases the number back to 0.

    ```
    # #[macro_use] extern crate pathfinder;
    # use pathfinder::{Coordinate, Node};
    # fn main() {
    # let a = node!("A", 0, 0);
    # let mut b = node!("B", 50, 50);
    # b.link(&a);
    b.disconnect();
    assert_eq!(b.get_link_avail_index(), 0);
    # }
    ```
     */
    pub fn get_link_avail_index(&self) -> usize {
        self.links()
            .iter()
            .position(|x| !x.is_connected())
            .or(Some(consts::MAX_LINKS - 1))
            .unwrap()
    }

    /**
    Removes all connects leaving this node. This still leaves connections going towards this node.


    ## Examples

    Connects two nodes and then disconnects it.

    ```
    # #[macro_use] extern crate pathfinder;
    # use pathfinder::{Coordinate, Node};
    # fn main() {
    let a = node!(0, 0);
    let mut b = node!(50, 50);
    b.link(&a);
    assert!(b.hl(0).is_ok());
    b.disconnect();
    assert!(b.hl(0).is_err());
    # }
    ```

    A node does not need to be connected, before attempted to disconnect it.

    ```
    # #[macro_use] extern crate pathfinder;
    # use pathfinder::{Coordinate, Node};
    # fn main() {
    let mut a = node!(0, 0);
    a.disconnect();
    # }
    ```

     */
    pub fn disconnect(&mut self) {
        self.links = [HL::new(0, 0); consts::MAX_LINKS];
    }

    /**
    Links Node self to another point that has Hash and Location implemented.


    ## Examples

    Connects two nodes, and verifies that they are connected.

    ```
    # #[macro_use] use pathfinder::*;
    # fn main() {
    let b = node!();
    let mut a = node!();
    a.link(&b);
    assert!(a.is_directly_connected(&b));
    # }
    ```

    Connects a node with a group.

    ```
    # #[macro_use] use pathfinder::*;
    # fn main() {
    let b = cluster!();
    let mut a = node!();
    a.link(&b);
    # }
    ```

     */
    pub fn link<P: Hash + Location>(&mut self, other: &P) {
        let i = self.get_link_avail_index();
        self.links[i] = HL {
            style: EdgeStyle::default(),
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
            style: EdgeStyle::default(),
            f,
            t,
            from: None,
            to: None,
        }
    }

    /**
    Sets the algorithm the edge will use to be drawn.

    Check out the EdgeStyle enum for all alternatives.


    ## Examples

    Sets the style to be 'straight' meaning sharp like an L shape.

    ```
    # #[macro_use] use pathfinder::*;
    # fn main() -> std::io::Result<()> {
    let b = cluster!();
    let mut a = node!();
    a.link(&b);
    a.hl_mut(0)?.style(EdgeStyle::Straight);
    # Ok(())
    # }
    ```

     */
    pub fn style(&mut self, style: EdgeStyle) {
        self.style = style;
    }

    /**
    Checks if the HL has two endpoint hashes.


    ## Examples

    ```
    # #[macro_use] use pathfinder::*;
    # fn main() -> std::io::Result<()> {
    let b = cluster!();
    let mut a = node!();
    a.link(&b);
    assert!(a.hl(0)?.is_connected());
    # Ok(())
    # }
    ```
     */
    pub fn is_connected(&self) -> bool {
        self.f != 0 && self.t != 0
    }

    /**
    Draws the HL on an Image Wrapper.

    Will not draw the Edge if it is not connected, or if the the HL's from and to connections are the same Node.

    Size increases drawing time with a squared factor.
     */
    fn draw(&self, mut image: IW, mut offset: Coordinate, size: u32) -> IW {
        let (mut from, mut to) = self.min_max();
        if !self.is_connected() || from == to {
            return image;
        }
        let s = coordinate!(size / 2);
        offset += s;
        from += offset;
        to += offset;

        for i in 0..size {
            for j in 0..size {
                let add = coordinate!(j, i) - s - s;
                let col = (size - i) as u8 * consts::DEFAULT_SHADE as u8;
                let plot = match self.style {
                    EdgeStyle::Direct => {
                        tools::plot_type(from + add, to + add, &tools::plot_bresenham)
                    }
                    EdgeStyle::Straight => {
                        tools::plot_type(from + add, to + add, &tools::plot_rectangle)
                    }
                    EdgeStyle::Ellipse => {
                        tools::plot_type(from + add, to + add, &tools::plot_ellipse)
                    }
                };
                let _ = plot
                    .iter()
                    .map(|c| image.put(c, image::Rgba([col, col, col, u8::max_value()])))
                    .collect::<Vec<_>>();
            }
        }
        image
    }
}

impl Group {
    /**
    Constructs a new Group.

    name is converted internally as a hash.
     */
    pub fn new(name: &str, coordinates: Coordinate) -> Self {
        Group {
            settings: Node::new(name, coordinates),
            nodes: Vec::new(),
        }
    }

    /**
    Adds a Node dynamically to the Group.


    ## Examples

    ```
    # #[macro_use] use pathfinder::*;
    # fn main() {
    let a = cluster!();
    let mut group = cluster!();
    group.new_node();
    # }
    ```
     */
    pub fn new_node(&mut self) {
        group::add_node(self, None, None, None);
    }

    /**
    Set the radius for the group's meta-data.


    ## Examples

    ```
    # #[macro_use] use pathfinder::*;
    # fn main() {
    let a = cluster!();
    let mut group = cluster!();
    group.radius(100);
    # }
    ```
     */
    pub fn radius(&mut self, radius: u32) {
        self.settings.radius = Some(radius);
    }

    /**
    Retrieves the nodes drawing in the group. Positions are relative to the group.


    ## Examples

    ```
    # #[macro_use] use pathfinder::*;
    # fn main() {
    let group = cluster!();
    let mut group = cluster!();
    group.add(50);
    assert_eq!(group.nodes().len(), 50);
    # }
    ```
     */
    pub fn nodes(&self) -> &Vec<Node> {
        &self.nodes
    }

    /**
    Retrieves the group meta data, used for setting properties such as hash and color.


    ## Examples

    ```
    # #[macro_use] use pathfinder::*;
    # fn main() {
    let group = cluster!();
    let mut group = cluster!();
    group.set().hash = 105;
    assert_eq!(group.hash(), 105);
    # }
    ```
     */
    pub fn set(&mut self) -> &mut Node {
        &mut self.settings
    }

    /**
    Adds a set of nodes randomly located inside the group's radius.


    ## Examples

    ```
    # #[macro_use] use pathfinder::*;
    # fn main() {
    let group = cluster!();
    let mut group = cluster!();
    group.add(500);
    assert_eq!(group.nodes().len(), 500);
    # }
    ```
     */
    pub fn add(&mut self, nr: u32) {
        for _ in 0..nr {
            let co = coordinate::gen_within_radius(self.position(), self.size());
            let mut node = node!(co);
            node.color = self.gen_color(co);
            self.push(node);
        }
    }

    /**
    Applies the closure over each mutable child node.


    ## Examples

    Prints all the nodes positions.

    ```
    # #[macro_use] use pathfinder::*;
    # fn main() {
    let mut group = cluster!();
    group.add(10);
    group.each(&|node: &mut Node| println!("{}", node.position()));
    # }
    ```

    Since it returns a mutable reference, It is more adapted for modifying the nodes.

    ```
    # #[macro_use] use pathfinder::*;
    # fn main() {
    # let mut group = cluster!();
    # group.add(10);
    group.each(&|node: &mut Node| node.geo = coordinate!());
    # }
    ```
     */
    pub fn each(&mut self, func: &Fn(&mut Node)) {
        for node in self.nodes.iter_mut() {
            func(node);
        }
    }

    /**
    Sets the color of the Group.
     */
    pub fn color(&mut self, rgba: image::Rgba<u8>) {
        self.settings.color = rgba;
    }

    /**
    Plots node according to the fn provided.

    The closure parameter is the number of children the group has.


    ## Examples

    ```
    # #[macro_use] use pathfinder::*;
    # fn main() {
    let mut group = cluster!();
    group.node_plot(&|u| coordinate!(u));
    # }
    ```
     */
    pub fn node_plot(&mut self, calc: &Fn(usize) -> Coordinate) {
        let c = coordinate::calc(self.position(), self.nodes.len(), calc);
        let color = self.gen_color(c);
        let mut node = node!(c);
        node.color = color;
        self.push(node);
    }

    /**
    Adds a Node with a specific minimum and maximum distance from the center of the Group.


    ## Examples

    ```
    # #[macro_use] use pathfinder::*;
    # fn main() {
    let mut group = cluster!();
    group.new_node_min_max(50, 60);
    assert!(group.nodes()[0].geo.lt(61));
    # }
    ```
     */
    pub fn new_node_min_max(&mut self, min: u32, max: u32) {
        group::add_node(self, None, Some(min), Some(max));
    }

    /**
    Removes all non-essentials from the standard implementation.


    ## Examples

    The cluster macro allows for this exact invocation.

    ```
    # #[macro_use] use pathfinder::*;
    # fn main() {
    let mut group = cluster!(10, 10);
    let group2 = Group::new_simple(10, 10);
    assert_eq!(group.position(), group2.position());
    # }
    ```
     */
    pub fn new_simple(x: i16, y: i16) -> Self {
        Group::new(&(x + y).to_string(), Coordinate::new(x, y))
    }

    /**
    Pushes a Node to the Group.


    ## Examples

    ```
    # #[macro_use] use pathfinder::*;
    # fn main() {
    let mut group = cluster!();
    let node = node!();
    group.push(node);
    assert_eq!(group.nodes().len(), 1);
    # }
    ```
     */
    pub fn push(&mut self, mut node: Node) {
        node.geo -= self.position();
        self.nodes.push(node);
    }

    /**
    Returns a dynamic radius based on the number of Nodes in the Group.

    It returns a larger number if it has more nodes.
     */
    pub fn dynamic_radius(&self) -> u32 {
        match self.settings.radius {
            Some(x) => x,
            None => u32::from(consts::DEFAULT_SIZE) + self.nodes.len() as u32 / 2,
        }
    }

    /**
    Rotates all the nodes inside the group.
     */
    pub fn rotate(&mut self, rad: f64) {
        // Use 0, 0 because the self.nodes positions relative.
        coordinate::rotate_around_axis(coordinate!(), &mut self.nodes, rad);
    }

    /**
    Generate a image::Rgba based on the color of the Group and the distance from center.

    This is useful to make nodes places in groups, but outside it's radius or close to it's radius appear as darker.


    ## Examples

    ```
    # #[macro_use] use pathfinder::*;
    # fn main() {
    let mut cluster = cluster!();
    cluster.radius(10);
    cluster.color(image::Rgba {data: [100, 100, 100, 255]});
    let rgba = cluster.gen_color(coordinate!(10, 10));
    assert!(rgba.data[0] < 50);
    # }
    ```
     */
    pub fn gen_color(&self, coordinates: Coordinate) -> image::Rgba<u8> {
        tools::range_color(
            self.dynamic_radius() as i16,
            self.settings.color,
            self.settings.geo,
            coordinates,
        )
    }

    /**
    Converts a list of tuples (x,y) to a Vector of Groups.
    Names are assigned from "A" and upwards automatically.


    ## Examples

    ```
    # use pathfinder::Group;
    let list = [(0, 0), (10, 10), (15, 15)];
    let groups = Group::from_list(&list);
    assert_eq!(groups.len(), 3);
    ```
     */
    pub fn from_list(list: &[(i16, i16)]) -> Vec<Self> {
        coordinate::from_list(&list, &|c, i| {
            Group::new(&std::char::from_u32(65 + i as u32).unwrap().to_string(), c)
        })
    }

    /**
    Link together groups.


    ## Examples

    ```
    # use pathfinder::*;
    let b: Group = Group::new("B", Coordinate::new(100, 100));
    let mut a: Group = Group::new("A", Coordinate::new(0, 0));
    a.link(&b);
    ```
     */
    pub fn link(&mut self, other: &Group) {
        self.settings.link(&other.settings);
    }
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
    /**
    Creates a new map, no min_max are intially required and are generated automatically when calling Map::map.
     */
    pub fn new() -> Self {
        Map {
            image: None,
            add: coordinate!(),
        }
    }

    /**
    Saves the image to disk at the given Path.


    ## Examples

    ```
    # use pathfinder::*;
    # use std::path::Path;
    # fn main() -> std::io::Result<()> {
    let nodes = Node::from_list(&[(0, 0), (10, 10)]);
    Map::new()
    .map(&nodes)
    .save(Path::new("/tmp/example.png"))?;
    # Ok(())
    # }
    ```
     */
    pub fn save(self, path: &std::path::Path) -> Result<(), std::io::Error> {
        self.image.unwrap().image().save(path)
    }

    /**
    Consumes the Map and returns the ImageWrapper.
     */
    pub fn consume(self) -> IW {
        self.image.unwrap()
    }

    /**
    Maps any struct that has implemented Draw, on to an ImageBuffer.


    ## Examples

    ```
    # use pathfinder::*;
    let nodes: Vec<Node> = Node::from_list(&[(0, 0), (100, 100)]);

    // Add content to vectors.

    let mut map = Map::new();
    map = map.map(&nodes);
    ```
     */
    pub fn map<T: Draw + Location + Hash + MinMax>(self, element: &[T]) -> Self {
        self.map_filter(&element, &|_| true)
    }

    /**
    Maps the elements but with an added filter parameter to exclude elements.
     */
    pub fn map_filter<T: Draw + Location + Hash + MinMax>(
        self,
        element: &[T],
        filter: &Fn(&T) -> bool,
    ) -> Self {
        self.map_params(&element, &filter, &Shape::Square)
    }

    /**
    Maps the elements with a specified shape struct.
     */
    pub fn map_shape<T: Draw + Location + Hash + MinMax>(
        self,
        element: &[T],
        shape: &Shape,
    ) -> Self {
        self.map_params(&element, &|_| true, shape)
    }

    /**
    Maps the elements without stabalizing the positions on the canvas.
     */
    pub fn map_absolute<T: Draw + Location + Hash + MinMax>(mut self, element: &[T]) -> Self {
        if self.image.is_none() {
            let (image, _) = map::gen_map(&element);
            self.image = Some(IW { img: image });
        }
        self.map(element)
    }

    /**
    Maps the elements but with all added parameters.
     */
    pub fn map_params<T: Draw + Location + Hash + MinMax>(
        mut self,
        element: &[T],
        filter: &Fn(&T) -> bool,
        shape: &Shape,
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
    /**
    Calculates the path from node A to node B.


    ## Examples

    ```
    # use pathfinder::*;
    let nodes = Node::from_list(&[(0, 0), (10, 10), (20, 20), (30, 30)]);
    let mut nodes = Node::linked_list(nodes);
    let path = Network::new(nodes).path("A", "D").unwrap();
    assert_eq!(path.len(), 4);
    ```
     */
    pub fn path<'a>(&'a self, a: &str, b: &str) -> std::io::Result<Vec<Node>> {
        let mut path = map::network::path(self, b, a, &map::network::path_shortest_leg)?;
        path.reverse();
        Ok(path)
    }

    /**
    Mimics path behaviour but works in reverse, Meaning stepping back in the links.
     */
    pub fn path_rev<'a>(&'a self, a: &str, b: &str) -> std::io::Result<Vec<Node>> {
        map::network::path(self, a, b, &map::network::path_shortest_leg)
    }

    /**
    Returns if the given hash exists in the network.


    ## Examples

    ```
    # use pathfinder::*;
    let nodes = Node::from_list(&[(0, 0), (10, 10), (20, 20), (30, 30), (40, 40)]);
    let network = Network::new(nodes.clone());
    assert!(network.get("A").is_some());
    assert!(network.get("F").is_none());
    ```
     */
    pub fn get(&self, element: &str) -> Option<Node> {
        map::network::get(self, element)
    }
}
