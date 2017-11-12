extern crate image;

use std::path::Path;

use super::tools::constants;
use pathfinder::node::Node;
use pathfinder::node::nodelink::NodeLink;

use super::tools::util;

use image::{ImageBuffer, Rgba};

// Returns the difference between the lowest and highest x and y values, in that order.
pub fn gen_map_dimensions(min_max: ((i16, i16), (i16, i16))) -> (u32, u32) {
    let x = min_max.0;
    let y = min_max.1;
    ((x.1 - x.0) as u32, (y.1 - y.0) as u32)
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

pub fn map_node_and_links(mut nodes: &mut [Node], links: &[NodeLink]) {

    // Specifies the max width that a text can use up.
    let node_name_length: u32 = 100;

    // Node size.
    let node_size: u32 = 8;

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
    map_nodes(&mut imgbuf, &mut nodes, add, node_size);

    // Draws all links
    map_links(&mut imgbuf, &links, add, node_size);

    if constants::DEBUGMODE {
        println!("Placed: {} Nodes and {} Links", nodes.len(), links.len());
    }

    // Save the image to local storage.
    let _ = imgbuf.save(&Path::new("examples/example2.png"));
}

pub fn map_nodes(mut image: &mut ImageBuffer<Rgba<u8>, Vec<u8>>, nodes: &mut [Node],  add: (i16, i16), node_size: u32) {

    // Iterate over the coordinates and pixels of the image
    for node in nodes {

        // Node
        let mut primary: Rgba<u8> = Rgba {data: [0,0,0,255]};

        // Color of the node.
        for i in 0..4 {
            let v = primary.data[i] as u32 + util::roll(0,255);

            // If v goes above what a u8 can take. Set it to max.
            let v2 = if v > 255 {
                255
            } else {
                v
            };

            primary.data[i] = v2 as u8;
        }

        node.set_color(primary);
        node.draw(&mut image, add.0 as u32, add.1 as u32, node_size);

    }

}

pub fn map_links(image: &mut ImageBuffer<Rgba<u8>, Vec<u8>>, links: &[NodeLink], add: (i16, i16), node_size: u32) {

    /*
    // Sets the scaling propety using an anonymous function for links.
    let scale = |a: i16, b: i16| {
        let mut res: f64 = 0.0;
        if b != 0 {
            res = a.abs() as f64 / b.abs() as f64;
        }
        res
    };
    */

    // Iterate over the coordinates and pixels of the image
    for link in links {

        // Coordinates to travel between.
        let from = link.from;
        let to = link.to;

        let color = from.color;

        // Starting positions.
        let mut x = (from.geo.x + add.0 + (node_size/2) as i16) as i16;
        let mut y = (from.geo.y + add.1 + (node_size/2) as i16) as i16;

        // Finish positions.
        let to_x = (to.geo.x + add.0 + (node_size/2) as i16) as i16;
        let to_y = (to.geo.y + add.1 + (node_size/2) as i16) as i16;

        let mut pos = Vec::new();

        // Keep putting pixels until they reach the destination.
        while x != to_x || y != to_y {

            let xa: u32 = x as u32;
            let ya: u32 = y as u32;

            pos.push((xa, ya));

            if x < to_x {
                x+=1;
            } else if x > to_x {
                x-=1;
            }

            if y < to_y {
                y+=1;
            } else if y > to_y {
                y-=1;
            }
        }

        // Places the pixels.
        for c in pos.iter() {
            image.put_pixel(c.0, c.1, color);
        }
    }

}