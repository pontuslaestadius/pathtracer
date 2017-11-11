use std::io;
use std::io::prelude::*;
use std::str::FromStr;
use std::fs::{OpenOptions, File};

use super::Node;
use super::super::tools::{constants, util};

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

    pub fn link(list: &[Node]) -> Vec<NodeLink> {
        let mut connections: Vec<NodeLink> = Vec::new();

        let range = list.len();
        for i in 0..range {
            // If you are on the last item in the list, There is nothing to link.

            let from = list.get(i).unwrap();

            let mut roll: usize = util::roll(0, (range/2) as u32) as usize;

            if i + roll >= range {
                roll = range -1 -i;
            }

            if i == range -1 {
                break;
            }
            let to = list.get(i +roll).unwrap();

            let link = NodeLink::new(from, to, true);
            connections.push(link);
        }

        for i in 0..range {
            // If you are on the last item in the list, There is nothing to link.

            if util::roll(0,100) > 70 {
                continue;
            }

            let from = list.get(i).unwrap();

            let mut roll: usize = util::roll(0, (range/2) as u32) as usize;

            if i + roll >= range {
                roll = range -1 -i;
            }

            if i == range -1 {
                break;
            }
            let to = list.get(i +roll).unwrap();

            let link = NodeLink::new(from, to, true);
            connections.push(link);
        }

        connections
    }

    pub fn save(&self) {

        // Opens the node file with specific OpenOptions.
        let mut file: File = OpenOptions::new()
            .create(true)
            .append(true)
            .truncate(false)
            .open(constants::LINKPATH)
            .unwrap();

        let omni = match self.omnidirectional {
            true => "true",
            _ => "false",
        };

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
            .open(constants::LINKPATH)
            .unwrap();

        let mut contents = String::new();
        let _ = file.read_to_string(&mut contents);
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

        Err(io::Error::new(io::ErrorKind::Other, "link does not exist in node list"))
    }
}

impl<'a> PartialEq for NodeLink<'a> {
    fn eq(&self, other: &NodeLink) -> bool {
        (self.from == other.from) &&
            (self.to == other.to) &&
            (self.omnidirectional == other.omnidirectional)
    }
}
