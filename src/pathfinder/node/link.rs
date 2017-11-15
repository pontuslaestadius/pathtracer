use std::io;
use std::io::prelude::*;
use std::str::FromStr;
use std::fs::{OpenOptions, File};

use super::Node;
use super::super::tools::{constants, util};
use image::{ImageBuffer, Rgba};

/*
     Link
     --------
     Holds connections between two struct with coordinates.
 */

pub struct Link<'a, T: 'a> {
    pub from: &'a T,
    pub to: &'a T,
}

impl<'a, T> NodeLink<'a, T> {

    pub fn draw(&self, image: &mut ImageBuffer<Rgba<u8>, Vec<u8>>, x_offset: i16, y_offset: i16, size: u32) {
        // TODO implement.
    }

    /// Creates a new nodeLink and binds two nodes together.
    pub fn new<'b>(from: &'b Node, to: &'b Node, ) -> NodeLink<'b> {
        NodeLink {
            from,
            to,
        }
    }

    /// Links a list of provided nodes randomly.
    pub fn link(list: &[Node]) -> Vec<NodeLink> {
        let mut connections: Vec<NodeLink> = Vec::new();

        let range = list.len();
        for i in 0..range/2 {
            // If you are on the last item in the list, There is nothing to link.

            let from = list.get(i*2).unwrap();

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
}

impl<'a> PartialEq for NodeLink<'a> {
    fn eq(&self, other: &NodeLink) -> bool {
        (self.from == other.from) &&
            (self.to == other.to) &&
            (self.omnidirectional == other.omnidirectional)
    }
}
