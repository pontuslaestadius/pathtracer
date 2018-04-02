use image::{ImageBuffer, Rgba};
use self::coordinates::Coordinate;
use std::cmp::PartialEq;
use std::fs::{File, OpenOptions};
use std::io;
use std::io::prelude::*;
use super::tools::{constants, util};

pub mod coordinates;
pub mod nodelink;
pub mod link;
pub mod figure;


/*
    Node
    ----
    Nodes represents anchors on a map that are the main focus.
*/

pub struct Node {
    pub name: String,
    pub geo: coordinates::Coordinate,
    pub color: Rgba<u8>,
    radius: Option<u32>,
}


// Setters and getters.
impl Node {
    pub fn get_name(&self) -> &String {
        &self.name
    }

    pub fn set_name(&mut self, name: String){
        self.name = name;
    }

    pub fn get_geo(&self) -> &Coordinate {
        &self.geo
    }

    pub fn set_geo(&mut self, geo: Coordinate){
        self.geo = geo;
    }

    pub fn get_color(&mut self) -> Rgba<u8> {
        self.color
    }

    pub fn set_color(&mut self, color: Rgba<u8>) {
        self.color = color;
    }

    pub fn get_radius(&self) -> Option<u32> {
        self.radius
    }

    pub fn set_radius(&mut self, radius: u32) {
        self.radius = Some(radius);
    }
}

impl Node {


    // Draw node.
    pub fn draw(&self, image: &mut ImageBuffer<Rgba<u8>, Vec<u8>>, x_offset: u32, y_offset: u32, size: u32) {

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
                image.put_pixel(x +i +1, y +j +1, Rgba {data: [0,0,0,255]});
            }
        }


    }

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

    */

    pub fn new(name: String, geo: coordinates::Coordinate) -> Node {
        Node {
            name,
            geo,
            color: Rgba {data: [0,0,0,255]},
            radius: None,
        }
    }

    // Saves the node to a text file.
    pub fn save(&self) -> Result<(), io::Error> {

        // Opens the node file.
        let mut file: File = match OpenOptions::new()
            .create(true)
            .append(true)
            .truncate(false)
            .open(constants::NODEPATH) {
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
            .open(constants::NODEPATH)
            .unwrap();

        let mut contents = String::new();
        let _ = file.read_to_string(&mut contents);
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

        Node::new(name, coordinates::Coordinate::new(x,y))
    }

    pub fn save_list(list: &[Node]) -> Result<(), io::Error> {

        // Opens the node file.
        let mut file: File = OpenOptions::new()
            .create(true)
            .append(true)
            .truncate(false)
            .open(constants::NODEPATH)?;

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
            geo: self.geo.clone(),
            color: self.color.clone(),
            radius: self.radius.clone(),
        }
    }
}

impl PartialEq for Node {
    fn eq(&self, other: &Node) -> bool {
        (self.geo == other.geo) && (self.name == other.name)
    }
}

// Opens
pub fn get_node_names() -> Result<Vec<String>, io::Error> {
    let mut file = File::open(constants::NAMEPATH)?;

    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let split = contents.split('\n');

    let mut names: Vec<String> = Vec::new();

    for value in split {
        names.push(value.to_string());
    }
    Ok(names)
}


