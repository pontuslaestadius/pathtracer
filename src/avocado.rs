pub mod node {

    extern crate rand;

    use rand::distributions::{IndependentSample, Range};

    use std::cmp::PartialEq;

    use std::fs::File;
    use std::fs::OpenOptions;

    use std::io::prelude::*;
    use std::io;

    use std::f64;


    pub struct Node {
        name: String,
        connections: Vec<TravelLeg>,
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

            let mut connections: String = String::new();
            connections.push_str("/");

            for leg in &self.connections {
                connections.push_str(leg.node.gen_id().as_str());
            }


            let str = [
                self.gen_id().as_str(),
                ",",
                self.name.as_str(),
                ",",
                connections.as_str(),
                ",",
                self.geo.x.to_string().as_str(),
                ",",
                self.geo.y.to_string().as_str(),
                "\n"
            ].concat();

            print!("Saving: {}", str.as_str());

            file.write_all(str.as_bytes()).expect("Couldn't save node");
        }

        // Creates an identifiable id for the Node.
        pub fn gen_id(&self) -> String {
            let mut id_x = self.geo.x.to_string();
            let mut id_y = self.geo.y.to_string();

            let mut len_x = id_x.len() -2;
            let mut len_y = id_y.len() -2;

            // TODO this looks horrible. Fix this.
            if len_x < 2 {
                len_x = id_x.len();
            }

            if len_y < 2 {
                len_y = id_y.len();
            }

            let format_x: String = id_x.split_off(len_x);
            let format_y: String = id_y.split_off(len_y);

            let mut clone = self.name.clone();

            clone.split_off(4);

            [
                clone,
                format_x,
                format_y
            ].concat()
        }

        // Connects two nodes by storing a TravelLeg in both of them.
        pub fn link(&mut self, mut other: Node) {
            let y_diff: u32 = ((self.geo.y - other.geo.y)^2) as u32;
            let x_diff: u32 = ((self.geo.x - other.geo.x)^2) as u32;
            let distance = ((y_diff + x_diff)/*^0.5*/) as u32; // TODO this is commented out just so it compiles.

            self.push_leg(
                TravelLeg {
                    node: other.clone(),
                    distance
                }
            );

            other.push_leg(
                TravelLeg {
                    node: self.clone(),
                    distance
                }

            );

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
            let mut split = contents.split('\n');


            for row in split {
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

            let gen_id = split.next(); // Useless
            let name = split.next().unwrap().to_string();
            let connections = split.next(); // TODO handle multiple connections.
            let x = split.next().unwrap().parse::<i16>().unwrap();
            let y = split.next().unwrap().parse::<i16>().unwrap();

            Node {
                name,
                connections: Vec::new(),
                geo: Coordinates {
                    x,
                    y
                }
            }

        }

        pub fn push_leg(&mut self, leg: TravelLeg) {
            self.connections.push(leg);
        }
    }


    impl Clone for Node {
        fn clone(&self) -> Node {
            Node {
                name: self.name.clone(),
                connections: self.connections.clone(),
                geo: self.geo.clone()
            }
        }
    }

    impl Node {
        pub fn new(name: String, geo: Coordinates) -> Node {
            Node {
                name,
                connections: Vec::new(),
                geo,
            }
        }
    }

    /*
        Travel Leg
    */

    pub struct TravelLeg {
        node: Node,
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

    /*
        Coordinates
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

        // TODO fix circle math.
        pub fn gen_within_radius(coord: Coordinates, radius: i16) -> Coordinates {
            let mut rng = rand::thread_rng();

            // Randomly gets the radius of the circle.
            let between: Range<i16> = Range::new(10, radius);
            let r = between.ind_sample(&mut rng) as f64;


            // gets a point on the circle's circumfrence.


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

}


