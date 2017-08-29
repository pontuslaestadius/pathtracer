pub mod node {

    extern crate rand;

    use rand::distributions::{IndependentSample, Range};

    use std::cmp::PartialEq;
    use std::cmp::Ordering;

    use constants::*;

    use std::fs::OpenOptions;

    use std::io::prelude::*;
    use std::io;

    use std::f64;
    use std::str::FromStr;

    use std::fs::File;
    use std::path::Path;

    use coordinates::Coordinates;

    /*
        Node
        ----
        Nodes represents anchors on a map that are the main focus.
    */

    pub struct Node {
        pub name: String,
        pub geo: Coordinates,
    }

    impl Node {

        /*

        fn draw_node(x: u32, y: u32) {
            let luma_background = 150;

            img.put_pixel(x, y, luma_background);

            img.put_pixel(x +2, y +2, luma_background);
            img.put_pixel(x +2, y -2, luma_background);
            img.put_pixel(x -2, y +2, luma_background);
            img.put_pixel(x -2, y -2, luma_background);

            for i in 0..2 {
                img.put_pixel(x +2, y +i, luma_background);
                img.put_pixel(x +i, y +2, luma_background);
                img.put_pixel(x +i, y -2, luma_background);
                img.put_pixel(x -2, y +i, luma_background);
            }

        }
        */


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

        pub fn new(name: String, geo: Coordinates) -> Node {
            Node {
                name,
                geo,
            }
        }

        // Saves the node to a text file.
        pub fn save(&self) -> Result<(), io::Error> {

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

            file.write_all(str.as_bytes())?;
            Ok(())
        }

        // Creates an identifiable id for the Node.
        pub fn gen_id(&self) -> String {
            let dis = (self.geo.x/2) as i32; // TODO this causes overflow at times.

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

        pub fn save_list(list: &[Node]) -> Result<(), io::Error> {

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
    }


    impl Clone for Node {
        fn clone(&self) -> Node {
            Node {
                name: self.name.clone(),
                geo: self.geo.clone()
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
        pub from: &'a Node,
        pub to: &'a Node,
        pub omnidirectional: bool // Does the path go both ways?
    }

    impl<'a> NodeLink<'a> {
        pub fn new<'b>(from: &'b Node, to: &'b Node, omnidirectional: bool) -> NodeLink<'b> {
            NodeLink {
                from,
                to,
                omnidirectional
            }
        }

        // TODO it just randomly links nodes. It should only link nodes close to it.
        // TODO this is supid unoptimized, Should not be used.
        pub fn link(list: &[Node]) -> Vec<NodeLink> {

            // If the list is too short to create links.
            if list.len() < 2 {
                return Vec::new();
            }

            let mut connections: Vec<NodeLink> = Vec::new();

            let ll: u32 = list.len() as u32;

            let max_links = (ll * ll/2) as u32; // TODO can overflow.

            // ll-1 = For minimum case when 2 nodes are provided.
            // We don't want 2 connections between those.
            // ll * ll is the maximum case when all nodes are connected.
            let mut rng = rand::thread_rng();
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

            // TODO bad complexity. O^2. Fix it. note: It has been improved, but only slightly.
            for node in list.iter() {
                let id = node.gen_id();
                if from == id {
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

                } else if to == id {
                    for node2 in list.iter() {
                        if from == node.gen_id() {
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


pub mod map {

    extern crate image;
    extern crate rand;

    use std::fs::File;
    use std::path::Path;
    use std::f64;

    use constants;
    use util::debug_print;
    use pathfinder::node::Node;
    use pathfinder::node::NodeLink;

    use pathfinder::node::*;
    use rand::distributions::{IndependentSample, Range};

    // Returns the difference between the lowest and highest x and y values, in that order.
    pub fn gen_map_dimensions(min_max: ((i16, i16), (i16, i16))) -> (u32, u32) {
        let x = min_max.0;
        let y = min_max.1;
        ((x.1 - x.0) as u32, (y.1 - x.0) as u32)
    }

    pub fn gen_min_max(list: &[Node]) -> ((i16, i16), (i16, i16)) {
        // Finds the min and max nodes, for scaling and bounderies.
        let mut min_x = list[0].geo.x;
        let mut min_y = list[0].geo.y;
        let mut max_x = list[0].geo.x;
        let mut max_y = list[0].geo.y;

        // Iterates over the nodes and finds the minimum and maximum x and y values.
        for node in list.iter() {
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

        if constants::DEBUGMODE {
            println!("Max_x: {} Min_x: {} Max_y: {} Min_y: {}", max_x, min_x, max_y, min_y);
        }

        ((min_x, max_x), (min_y, max_y))
    }

    pub fn gen_stabalize(min_max: ((i16, i16), (i16, i16))) -> (i16, i16) {
        // Sets the additions requried to center the pixels on the map.

        let x = min_max.0;
        let y = min_max.1;

        (-x.0, -y.0)
    }

    pub fn map_node(list: &[Node]) {

        // Indicates the size of the node in pixels.
        let node_size = 4;

        let min_max = gen_min_max(list);

        let res = gen_map_dimensions(min_max);
        let add = gen_stabalize(min_max);

        // Sets the imag
        let mut imgx = (res.1 +2) as u32;
        let mut imgy = (res.0 +2) as u32;

        println!("Creating Nodemap with resolution: {}x{}", imgx, imgy);

        // Create a new ImgBuf with width: imgx and height: imgy
        let mut imgbuf = image::ImageBuffer::new(imgx, imgy);
        let luma = image::Luma([200 as u8]);
        let luma_background = image::Luma([50 as u8]);

        // Counts the number of nodes placed.
        let mut placed_nodes = 0; // TODO this wont be required once the map is 100% functioning.

        // Adds background nodes first.
        let mut rng = rand::thread_rng();
        let between: Range<u32> = Range::new((list.len()/2) as u32, list.len() as u32);
        let mut range = between.ind_sample(&mut rng) as u32;

        for _ in 0..range {
            let between: Range<u32> = Range::new(0, imgx);
            let roll_x = between.ind_sample(&mut rng) as u32;

            let between: Range<u32> = Range::new(0, imgy);
            let roll_y = between.ind_sample(&mut rng) as u32;

            imgbuf.put_pixel(roll_x, roll_y, luma_background);
        }

        // Iterate over the coordiantes and pixels of the image
        for node in list {
            // println!("x: node.geo.x: {} add_x: {} y: node.geo.y: {} add_y: {}", node.geo.x, add_x, node.geo.y, add_y);
            let mut x = ((node.geo.x + add.0) as i16); // TODO can overflow
            let mut y = (node.geo.y + add.1) as i16; // TODO can overflow

            //Node::draw_node(&imgbuf, x as u32, y as u32);

            for i in 0..node_size {
                for j in 0..node_size {
                    imgbuf.put_pixel((x+i) as u32, (y+j) as u32, luma);
                }
            }

            placed_nodes += 1;
        }

        // Save the image
        let fout = &mut File::create(&Path::new("resources/nodemap.png")).unwrap();

        println!("Placed: {} Nodes", placed_nodes);

        // We must indicate the image’s color type and what format to save as
        let _    = image::ImageLuma8(imgbuf).save(fout, image::PNG);
    }

    pub fn map_node_and_links(nodes: &[Node], links: &[NodeLink]) {
        // Indicates the size of the node in pixels.
        let node_size: u32 = 4;

        let min_max = gen_min_max(nodes);

        let res = gen_map_dimensions(min_max);
        let add = gen_stabalize(min_max);

        // Sets the imag
        let mut imgx = (res.0 + node_size*2) as u32;
        let mut imgy = (res.1 + node_size*2) as u32;

        println!("Creating map_node_and_links with resolution: {}x{}", imgx, imgy);

        // Create a new ImgBuf with width: imgx and height: imgy
        let mut imgbuf = image::ImageBuffer::new(imgx, imgy);

        let luma_node = image::Luma([230 as u8]);
        let luma_background_node = image::Luma([20 as u8]);

        // Counts the number of nodes placed.
        let mut placed_links = 0; // TODO this wont be required once the map is 100% functioning.
        let mut placed_nodes = 0; // TODO this wont be required once the map is 100% functioning.


        /*

        // Adds background nodes first.
        let mut rng = rand::thread_rng();
        let between: Range<u32> = Range::new((nodes.len()/2) as u32, nodes.len() as u32);
        let mut range = between.ind_sample(&mut rng) as u32;

        for _ in 0..range {
            let between: Range<u32> = Range::new(0, imgx);
            let roll_x = between.ind_sample(&mut rng) as u32;

            let between: Range<u32> = Range::new(0, imgy);
            let roll_y = between.ind_sample(&mut rng) as u32;

            imgbuf.put_pixel(roll_x, roll_y, luma_background_node);
        }
        */

        // Iterate over the coordinates and pixels of the image
        for link in links {
            //let mut x = ((node.geo.x + add.0) as i16); // TODO can overflow
            //let mut y = (node.geo.y + add.1) as i16; // TODO can overflow

            let luma_link = image::Luma([roll(60,100) as u8]);

            let from = link.from;
            let to = link.to;

            let from_x = (from.geo.x + add.0 + (node_size/2) as i16) as i16;
            let from_y = (from.geo.y + add.1 + (node_size/2) as i16) as i16;

            let to_x = (to.geo.x + add.0 + (node_size/2) as i16) as i16;
            let to_y = (to.geo.y + add.1 + (node_size/2) as i16) as i16;

            let dif_x = from_x - to_x;
            let dif_y = from_y - to_y;

            let scale = |a: i16, b: i16| {
                let mut res: f64 = 0.0;
                if b != 0 {
                    res = (a.abs() as f64 / b.abs() as f64);
                }
                res
            };

            let mut scale_y: f64 = scale(dif_x, dif_y);

            let mut scale_x: f64 = scale(dif_y, dif_x);


            let mut x = from_x;
            let mut y = from_y;

            println!("to ({},{})", to_x, to_y);
            println!("fr ({},{})", from_x, from_y);

            let mut x_ite: f64 = 0.0;
            let mut y_ite: f64 = 0.0;


            let mut inc_x = 1;
            if x > to_x {
                inc_x = -1;
            }
            let mut inc_y = 1;
            if y > to_y {
                inc_y = -1;
            }

            while x != to_x || y != to_y {
                print!("-");
                x_ite += scale_x;
                y_ite += scale_y;

                if x == to_x {
                    inc_x = 0;
                }
                if y == to_y {
                    inc_y = 0;
                }

                if          x_ite >= 1.0 || y_ite > 1.0 {
                    x += inc_x;
                    y += inc_y;
                    x_ite -= 1.0;
                    y_ite -= 1.0;
                } else if   y_ite >= 1.0 {
                    y += inc_y;
                    y_ite -= 1.0;
                } else if   x_ite >= 1.0 {
                    x += inc_x;
                    x_ite -= 1.0;
                } else {
                    println!("edge case like this should not occur"); // TODO this occurs from time to time.
                    continue;
                }

                imgbuf.put_pixel(x as u32, y as u32, luma_link);


                //imgbuf.put_pixel(x as u32, y as u32, luma_link);
            }

            placed_links += 1;
        }


        // Iterate over the coordiantes and pixels of the image
        for node in nodes {
            // println!("x: node.geo.x: {} add_x: {} y: node.geo.y: {} add_y: {}", node.geo.x, add_x, node.geo.y, add_y);
            let mut x = ((node.geo.x + add.0) as i16); // TODO can overflow
            let mut y = (node.geo.y + add.1) as i16; // TODO can overflow

            //x += (node_size/2) as i16;
            //y += (node_size/2) as i16;

            for i in 0..node_size +1 {
                for j in 0..node_size +1 {
                    if i == 0 || i == node_size || j == node_size || j == 0 {
                        imgbuf.put_pixel((x+(i + node_size/2) as i16) as u32, (y+(j + node_size/2) as i16) as u32, luma_node);
                    }
                }
            }

            placed_nodes += 1;
        }


        // Save the image
        let fout = &mut File::create(&Path::new("resources/node_and_link_map.png")).unwrap();

        println!("Placed: {} Nodes", placed_nodes);
        println!("Placed: {} Links", placed_links);

        // We must indicate the image’s color type and what format to save as
        let _    = image::ImageLuma8(imgbuf).save(fout, image::PNG);
    }

    fn roll(min: u32, max: u32) -> u32 {
        let mut rng = rand::thread_rng();
        let between: Range<u32> = Range::new(min, max);
        between.ind_sample(&mut rng) as u32
    }
}