//! Author: Pontus Laestadius
//! Version: 0.3
//! Since: 2017-12-02
//!
//! Visualizes a provided log with the marked tags.
//!

extern crate pathfinder;
extern crate rand;
extern crate image;
extern crate gif;

use gif::{Frame, Encoder, Repeat, SetParameter};
use gif::*;
use pathfinder::map::*;
use pathfinder::group::min_max;
use pathfinder::{map, data, group, node};
use pathfinder::*;
use std::env;
use std::fs::File;

fn main() {
    let ls = 35; // Letter spacing.
    let hs = ls/2; // Letter spacing half.

    // This is a "small" list of positions required to make a figure!
    let node_pos: Vec<(i16, i16)> = vec!(
        (0,0), (hs, -hs), (0, -ls), (0, ls),
        (ls, -hs), (ls+hs, ls), (hs, 0), (ls*2 +hs, 0),
        (ls*2, -hs), (ls*2, ls), (ls*2 +hs, ls),
        (ls*3, -hs), (ls*3, ls), (ls*3, hs), (ls*3 +hs, hs), (ls*3 +hs, ls),
        (ls*4, ls), (ls*4, -hs), (ls*4 +hs, -hs), (ls*4, -hs), (ls*4, 0), (ls*4 +hs, 0),
        (ls*5 -hs/2, hs/2), (ls*5 -hs/4, hs/2), (ls*5, hs/2), (ls*5, ls), (ls*5 +hs/2, ls),
        (ls*6 -hs, 0), (ls*6, ls), (ls*6 +hs/3, 0),
        (ls*7 -hs, -hs), (ls*7 -hs, ls), (ls*7, ls -hs), (ls*7 -hs, hs/2),
        (ls*8, 0), (ls*8 -hs, -hs), (ls*8 -ls, 0), (ls*8 -hs, ls), (ls*8 +hs, ls),
        (ls*9 -hs, -hs/3), (ls*9 -hs, 0), (ls*9, 0), (ls*9, hs/3)
    );


    // Node size.
    let node_size: u32 = 6;

    let mut image = File::create("hello_world.gif").unwrap();
    let width = 300;
    let height = 200;
    let mut encoder = Encoder::new(&mut image, width, height, &[]).unwrap();
    encoder.set(Repeat::Infinite).unwrap();

    for i in 0..50 {
        println!("loop {}", i);
        let mut nodes: Vec<Node<Square>> = Vec::new();

        // Add each position as a node.
        for pos in node_pos.iter() {
            let mut node = Node::new("example", Coordinate::new(pos.0,pos.1));
            node.radius = Some(node_size);
            node.color = tools::gen_rgba();
            nodes.push(node);
        }

        // Link them sequentially in order.
        let link_vec = sequentially_link_nodes(&nodes);

        let min_max = gen_min_max(&nodes);

        // Stabilizes the picture to have the action in the center of the image.
        let add = gen_stuff(min_max);

        // Generates an image buffer.
        let mut imgbuf =     image::DynamicImage::new_rgba8(width.into(), height.into()).to_rgba();

        // Draws all nodes.
        map_nodes(&mut imgbuf, &nodes, add, node_size);

        // Draws all links
        map_links(&mut imgbuf, &link_vec, add, node_size);

        let mut pixels: Vec<u8> = Vec::new();

        for pix in imgbuf.pixels() {
            pixels.push(pix.data[0]);
            pixels.push( pix.data[1]);
            pixels.push(pix.data[2]);
            pixels.push(pix.data[3]);
        }

        // Create frame from data
        let frame = gif::Frame::from_rgba(imgbuf.width() as u16, imgbuf.height() as u16, &mut pixels);
        encoder.write_frame(&frame).unwrap();

    }
}