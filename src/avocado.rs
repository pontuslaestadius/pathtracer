pub mod node {

    extern crate rand;

    use rand::distributions::{IndependentSample, Range};

    use std::cmp::PartialEq;

    use std::fs::File;
    use std::fs::OpenOptions;

    use std::io::prelude::*;
    use std::io;

    use std::f64;


    /*
        Node
        ----
        Nodes represents anchors on a map that are the main focus.
    */

    pub struct Node {
        name: String,
        geo: Coordinates,
    }

    impl Node {

        // Saves the node to a text file.
        pub fn save(&self) {
            let path = "nodes/nodes.txt";

            // Opens the node file.
            let mut file: File = match OpenOptions::new()
                .create(true)
                .append(true)
                .truncate(false)
                .open(path) {
                Result::Ok(t) => t,
                            _ => panic!("Couldn't open path"),
            };


            let str = [
                self.name.as_str(),
                ",",
                self.geo.x.to_string().as_str(),
                ",",
                self.geo.y.to_string().as_str(),
                "\n"
            ].concat();

            file.write_all(str.as_bytes()).expect("Couldn't save node");
        }

        // Creates an identifiable id for the Node.
        pub fn gen_id(&self) -> String {
            let dis = (self.geo.x / (self.geo.y/14)) as i32; // TODO this causes overflow at times.

            let mut clone = self.name.clone();

            clone.split_off(5);

            [
                clone,
                dis.to_string()
            ].concat()
        }

        // Loads and returns all saved nodes.
        pub fn load() -> Vec<Node> {
            let path = "nodes/nodes.txt";

            let mut nodes: Vec<Node> = Vec::new();

            let mut file = OpenOptions::new()
                .read(true)
                .open(path)
                .unwrap();

            let mut contents = String::new();
            file.read_to_string(&mut contents);
            let split = contents.split('\n');


            for row in split {
                // Ignores things like empty lines, are anything that may be invalid.
                if row.len() > 15 {
                    nodes.push(Node::parse(row));
                }
            }

            println!("loaded nodes: {}", nodes.len());


            nodes
        }

        pub fn parse(str: &str) -> Node {
            println!("Parsing: {}", str);

            let string: String = str.to_string();

            let mut split = string.split(",");

            let name = split.next().unwrap().to_string();
            let x = split.next().unwrap().parse::<i16>().unwrap();
            let y = split.next().unwrap().parse::<i16>().unwrap();

            Node {
                name,
                geo: Coordinates {
                    x,
                    y
                }
            }

        }

    }


    impl Clone for Node {
        fn clone(&self) -> Node {
            Node {
                name: self.name.clone(),
                geo: self.geo.clone()
            }
        }
    }

    impl Node {
        pub fn new(name: String, geo: Coordinates) -> Node {
            Node {
                name,
                geo,
            }
        }
    }

    impl PartialEq for Node {
        fn eq(&self, other: &Node) -> bool {
            (self.geo == other.geo) && (self.name == other.name)
        }
    }

    /*
        NodeLink
        --------
        Holds connections between nodes.
    */


    pub struct NodeLink<'a> {
        from: &'a Node,
        to: &'a Node,
        omnidirectional: bool // Does the path go both ways?
    }

    impl<'a> NodeLink<'a> {
        fn new<'b>(from: &'b Node, to: &'b Node, omnidirectional: bool) -> NodeLink<'b> {
            NodeLink {
                from,
                to,
                omnidirectional
            }
        }

        // TODO it just randomly links nodes. It should only link nodes close to it.
        pub fn link(list: &[Node]) -> Vec<NodeLink> {

            // If the list is too short to create links.
            if list.len() < 2 {
                return Vec::new();
            }

            let mut connections: Vec<NodeLink> = Vec::new();
            let mut rng = rand::thread_rng();

            let ll: u32 = list.len() as u32;

            let mut max_links = (ll * ll/2) as u32;

            // ll-1 = For minimum case when 2 nodes are provided.
            // We don't want 2 connections between those.
            // ll * ll is the maximum case when all nodes are connected.
            let between: Range<u32> = Range::new(ll*2, max_links); // TODO ll*5 doesn't mean anything. fix it.
            let mut range = between.ind_sample(&mut rng) as u32;

            let between: Range<u32> = Range::new(0, ll -1);

            for i in 0..range {

                // Because the nodes in the list are ordered based on proximity. Use a relative number
                // To link them.

                let distance = 4;

                let f = between.ind_sample(&mut rng) as usize;

                let from: &Node = &list[f];
                let mut s_range;

                {
                    let mut max: usize;
                    if (f + distance) > ll as usize {
                        max = (ll-1) as usize;
                    } else {
                        max = f + distance;
                    }

                    let mut min: usize;
                    if f < distance {
                        min = 0;
                    } else {
                        min = f - distance;
                    }

                    s_range = Range::new(min, max);
                }


                let t: usize = s_range.ind_sample(&mut rng) as usize;
                let to: &Node = &list[t];

                let temp = NodeLink::new(from, to, true); // TODO don't use true.

                // a connection can not be made between the same node.
                if from == to {

                    // Adds one to the range so the odd case of repeated same number
                    // Produces a lower number of links.
                    range += 1;
                    continue;
                }

                // TODO creates horrible complexity for big lists.
                // Such as: O^2
                // Ignores duplicate connections.
                let mut skip = false;
                for link in &connections {
                    if (link == &temp) {
                        // range += 1; // TODO commented out because it might cause issues.
                        skip = true;
                        break;
                    }
                }

                if !skip {
                    connections.push(temp);
                }

            }

            connections
        }

        pub fn save(&self) {

            let path = "nodes/links.txt";

            // Opens the node file.
            let mut file: File = match OpenOptions::new()
                .create(true)
                .append(true)
                .truncate(false)
                .open(path) {
                Result::Ok(t) => t,
                _ => panic!("Couldn't open path"),
            };

            let mut omni;

            if self.omnidirectional {
                omni = "true";
            } else {
                omni = "false";
            }

            let str = [
                self.from.gen_id().as_str(),
                ",",
                self.to.gen_id().as_str(),
                ",",
                omni,
                "\n"
            ].concat();

            file.write_all(str.as_bytes()).expect("Couldn't save node");


        }


        pub fn load(&self, list: &[Node]) -> Vec<NodeLink> {
            // TODO implement.
        }

    }

    impl<'a> PartialEq for NodeLink<'a> {
        fn eq(&self, other: &NodeLink) -> bool {
            (self.from == other.from) &&
                (self.to == other.to) &&
                (self.omnidirectional == other.omnidirectional)
        }
    }


    /*
        Travel Leg
        ----------
        Represents one leg of a journey.
    */

    /*

    pub struct TravelLeg<'a> {
        node: &'a Node,
        // time: u32, // TODO implentation, along with method of transport.
        distance: u32,
    }

    impl Clone for TravelLeg {
        fn clone(&self) -> TravelLeg {
            TravelLeg {
                node: self.node.clone(), // TODO I feel like this is a recursive call...
                distance: self.distance.clone()
            }
        }
    }

    */

    /*
        Coordinates
        -----------
        Stores an x and y coordinate representing a position on a map.
    */

    pub struct Coordinates {
        x: i16,
        y: i16,
    }

    impl PartialEq for Coordinates {
        fn eq(&self, other: &Coordinates) -> bool {
            (self.x == other.x) && (self.y == other.y)
        }
    }

    impl Coordinates {
        pub fn gen() -> Coordinates {
            let tuple = rand::random::<(i32, i32)>();

            Coordinates {
                x: rand::random::<i16>(),
                y: rand::random::<i16>(),
            }
        }

        pub fn gen_within_radius(coord: Coordinates, radius: i16) -> Coordinates {
            let mut rng = rand::thread_rng();

            // Randomly gets the radius of the circle.
            let between: Range<i16> = Range::new(10, radius);
            let r = between.ind_sample(&mut rng) as f64;


            // gets a point on the circle's circumference.
            let cir = |a: f64, b: f64| a + r * b;

            // Gets the Angle
            let between: Range<i16> = Range::new(0, 10);
            let angle = between.ind_sample(&mut rng);
            let a: f64 = f64::consts::PI * (0.1 * angle as f64);

            let x = cir(coord.x as f64, a.cos()) as i16;            // x = cx + r * cos(a)
            let y = cir(coord.y as f64, a.sin()) as i16;            // y = cy + r * sin(a)


            Coordinates {
                x,
                y
            }
        }
    }

    impl Clone for Coordinates {
        fn clone(&self) -> Coordinates {
            Coordinates {
                x: self.x,
                y: self.y
            }
        }
    }

    /*
        Network
        -------
        Binds the nodes and the connections via an extra layer of abstraction
    */

    pub struct Network<'a>  {
        wrappers: Vec<Wrapper<'a>>
    }

    impl<'a> Network<'a> {

        fn new(nodes: [Node], links: [NodeLink]) -> Network {
            let wrappers = Vec::new();

            // TODO implement

            Network (
                wrappers
            )
        }

    }


    /*
        Wrapper
        -------
        Wraps around the node and links and creates a correlation.
    */

    pub struct Wrapper<'a> {
        node: Node,
        links: Vec<NodeLink<'a>>
    }

    impl<'a> Wrapper<'a> {

        pub fn next(&self) -> Option<NodeLink> {
            self.links.next()
        }

    }


}


