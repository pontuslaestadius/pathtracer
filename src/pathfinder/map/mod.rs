extern crate image;

use std::path::Path;

use super::tools::constants;
use pathfinder::node::Node;
use pathfinder::group::*;
use pathfinder::node::nodelink::NodeLink;
use pathfinder::node::link::*;

use image::{ImageBuffer, Rgba};

// Returns the difference between the lowest and highest x and y values, in that order.
pub fn gen_map_dimensions(min_max: ((i16, i16), (i16, i16))) -> (u32, u32) {
    let x = min_max.0;
    let y = min_max.1;
    ((x.1 - x.0) as u32, (y.1 - y.0) as u32)
}

pub fn gen_min_max(list: &[Node]) -> ((i16, i16), (i16, i16)) {
    // Finds the min and max nodes, for scaling and boundaries.
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

    ((min_x, max_x), (min_y, max_y))
}

// Sets the additions requried to center the pixels on the map.
pub fn gen_stabalize(min_max: ((i16, i16), (i16, i16))) -> (i16, i16) {

    let x = min_max.0;
    let y = min_max.1;

    (-x.0, -y.0)
}

// Generates a canvas and returns it.
pub fn gen_canvas(w: u32, h: u32) -> image::ImageBuffer<Rgba<u8>, Vec<u8>> {
    image::DynamicImage::new_rgba8(w, h).to_rgba()
}


pub fn groups_and_links(groups: &[Group], links: &[Link]) { // TODO imlpementing.
    // Node size.
    let node_size: u32 = 4;

    // Gets the highest and lowest of all the coordinates.
    let min_max = min_max(groups);

    // Gets the resolution of the image.
    let res = gen_map_dimensions(min_max);

    // Stabilizes the picture to have the action in the center of the image.
    let add = gen_stabalize(min_max);

    // Sets the image size.
    let width  = (res.0 + node_size*2) as u32;
    let height = (res.1 + node_size*2) as u32;

    // Create a new ImgBuf with width: imgx and height: imgy
    let mut imgbuf = gen_canvas(width, height);

    for group in groups.iter() {
        group.draw(&mut imgbuf, add.0 as u32, add.1 as u32, node_size);
    }

    // Save the image to local storage.
    let _ = imgbuf.save(&Path::new("examples/example2.png"));

}

pub fn map_groups(groups: &[Group]) {
    // Node size.
    let node_size: u32 = 4;

    let min_max = min_max(groups);

    // Gets the resolution of the image
    let res = gen_map_dimensions(min_max);

    // Stabilizes the picture to have the action in the center of the image.
    let add = gen_stabalize(min_max);

    // Sets the image size.
    let width  = (res.0 + node_size*2) as u32;
    let height = (res.1 + node_size*2) as u32;

    println!("Creating map_groups with resolution: {}x{}", width, height);

    // Create a new ImgBuf with width: imgx and height: imgy
    let mut imgbuf = gen_canvas(width, height);

    for group in groups.iter() {
        group.draw(&mut imgbuf, add.0 as u32, add.1 as u32, node_size);
    }

    // Save the image to local storage.
    let _ = imgbuf.save(&Path::new("examples/example2.png"));



}

pub fn map_node_and_links(nodes: &[Node], links: &[NodeLink]) {

    // Specifies the max width that a text can use up.
    let node_name_length: u32 = 100;

    // Node size.
    let node_size: u32 = 4;

    let min_max = gen_min_max(nodes);

    // Gets the resolution of the image
    let res = gen_map_dimensions(min_max);

    // Stabilizes the picture to have the action in the center of the image.
    let add = gen_stabalize(min_max);

    // Sets the image size.
    let width = (res.0 + node_size*2 +node_name_length) as u32;
    let height = (res.1 + node_size*2) as u32;

    println!("Creating map_node_and_links with resolution: {}x{}", width, height);

    // Create a new ImgBuf with width: imgx and height: imgy
    let mut imgbuf = gen_canvas(width, height);

    // Draws all nodes.
    map_nodes(&mut imgbuf, &nodes, add, node_size);

    // Draws all links
    map_links(&mut imgbuf, &links, add, node_size);

    if constants::DEBUGMODE {
        println!("Placed: {} Nodes and {} Links", nodes.len(), links.len());
    }

    // Save the image to local storage.
    let _ = imgbuf.save(&Path::new("examples/example2.png"));
}

pub fn map_nodes(mut image: &mut ImageBuffer<Rgba<u8>, Vec<u8>>, nodes: &[Node],  add: (i16, i16), node_size: u32) {

    // Iterate over the coordinates and pixels of the image
    for node in nodes {
        node.draw(&mut image, add.0 as u32, add.1 as u32, node_size);
    }

}

pub fn map_links(mut image: &mut ImageBuffer<Rgba<u8>, Vec<u8>>, links: &[NodeLink], add: (i16, i16), node_size: u32) {

    // Iterate over the coordinates and pixels of the image
    for link in links {
        link.draw(&mut image, add.0, add.1, node_size);
    }

}