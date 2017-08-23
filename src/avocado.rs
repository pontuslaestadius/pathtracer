pub mod node {

    extern crate rand;

    use rand::distributions::{IndependentSample, Range};

    use std::cmp::PartialEq;
    use std::cmp::Ordering;

    use std::fs::File;
    use std::fs::OpenOptions;

    use std::io::prelude::*;
    use std::io;

    use std::f64;
    use std::str::FromStr;



    /*
        Constants
    */

    pub static NODEPATH: &str = "resources/nodes.txt";
    pub static LINKPATH: &str = "resources/links.txt";
    pub static NAMEPATH: &str = "resources/nodenames.txt";

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

        /*

        pub fn sort(list: [Node]) {
            Node::rec_sort(list,0,list.len());
        }

        // A recursive sort implementation using divide and conquer.
        pub fn rec_sort(list: &mut [Node], start: u32, end: u32) -> [Node] {

            match end - start {
                // If there is only one item, It is already sorted.
                1 => return list,
                2 => {
                    if list[0].geo > list[1].geo
                        return [list[1], list[0]];
                    return [list[0], list[1]];
                }
                _ => {
                    let half = (list.len/2) as u32;
                    Node::rec_sort(list, 0, half);
                    Node::rec_sort(list, half, list.len());
                }

            }
            list
        }

        pub fn quick_sort(list: &mut [Node]) {
            // TODO implement.

        }
        */

        // Saves the node to a text file.
        pub fn save(&self) {

            // Opens the node file.
            let mut file: File = match OpenOptions::new()
                .create(true)
                .append(true)
                .truncate(false)
                .open(NODEPATH) {
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

            let mut nodes: Vec<Node> = Vec::new();

            let mut file = OpenOptions::new()
                .read(true)
                .open(NODEPATH)
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

            nodes
        }

        pub fn parse(str: &str) -> Node {

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

    pub fn save(list: &[Node]) -> Result<(), io::Error> {

        // Opens the node file.
        let mut file: File = OpenOptions::new()
            .create(true)
            .append(true)
            .truncate(false)
            .open(NODEPATH)?;

        for item in list {
            let str = [
                item.name.as_str(),
                ",",
                item.geo.x.to_string().as_str(),
                ",",
                item.geo.y.to_string().as_str(),
                "\n"
            ].concat();

            file.write_all(str.as_bytes())?;
        }
        Ok(())
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

            let max_links = (ll * ll/2) as u32;

            // ll-1 = For minimum case when 2 nodes are provided.
            // We don't want 2 connections between those.
            // ll * ll is the maximum case when all nodes are connected.
            let between: Range<u32> = Range::new(ll*2, max_links); // TODO ll*5 doesn't mean anything. fix it.
            let mut range = between.ind_sample(&mut rng) as u32;

            let between: Range<u32> = Range::new(0, ll -1);

            for _ in 0..range {

                // Because the nodes in the list are ordered based on proximity. Use a relative number
                // To link them.

                let distance = 4;

                let f = between.ind_sample(&mut rng) as usize;

                let from: &Node = &list[f];
                let s_range;

                {
                    let max: usize;
                    if (f + distance) > ll as usize {
                        max = (ll-1) as usize;
                    } else {
                        max = f + distance;
                    }

                    let min: usize;
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
                    if link == &temp {
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

            // Opens the node file.
            let mut file: File = OpenOptions::new()
                .create(true)
                .append(true)
                .truncate(false)
                .open(LINKPATH)
                .unwrap();

            let omni;

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

        pub fn load<'b>(&self, list: &'b [Node]) -> Vec<NodeLink<'b>> {
            let mut links: Vec<NodeLink> = Vec::new();

            let mut file = OpenOptions::new()
                .read(true)
                .open(LINKPATH)
                .unwrap();

            let mut contents = String::new();
            file.read_to_string(&mut contents);
            let split = contents.split('\n');

            for row in split {
                // Ignores things like empty lines, are anything that may be invalid.
                if row.len() > 15 {
                    let res = NodeLink::parse(row, &list).unwrap();
                    links.push(res);
                }
            }
            links
        }

        pub fn parse<'b>(str: &str, list: &'b [Node]) -> Result<NodeLink<'b>, io::Error> {
            let string: String = str.to_string();

            let mut split = string.split(",");

            let from = split.next().unwrap().to_string();
            let to = split.next().unwrap().to_string();
            let omni_parsed: bool = FromStr::from_str(
                split.next().unwrap()).unwrap();

            // Connect the Gen_id with nodes.

            // TODO bad complexity. O^2. Fix it.
            for node in list.iter() {
                if from == node.gen_id() {
                    for node2 in list.iter() {
                        if to == node.gen_id() {
                            return Ok(
                                NodeLink {
                                    from: &node,
                                    to: &node2,
                                    omnidirectional: omni_parsed
                                }
                            )
                        }
                    }

                    break;
                }
            }

            Err(
                io::Error::new(io::ErrorKind::Other, "Link does not in node list.")
            )
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
        Coordinates
        -----------
        Stores an x and y coordinate representing a position on a map.
    */

    #[derive(Eq)]
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

        pub fn new(x: i16, y: i16) -> Coordinates {
            Coordinates {
                x,
                y
            }
        }

        pub fn gen() -> Coordinates {
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

    impl Ord for Coordinates {
        fn cmp(&self, other: &Coordinates) -> Ordering {
            self.x.cmp(&other.x) // TODO only implements for x and not y.
        }
    }

    impl PartialOrd for Coordinates {
        fn partial_cmp(&self, other: &Coordinates) -> Option<Ordering> {
            Some(self.cmp(other))
        }
    }

    /*
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

    pub struct Wrapper {
        node: Node,
        links: Vec<NodeLink>
    }

    impl<'a> Wrapper<'a> {

        pub fn next(&self) -> Option<NodeLink> {
            self.links.next()
        }

        pub fn add(&self, link: NodeLink) {
            self.links.push(link);
        }

    }
*/

}


