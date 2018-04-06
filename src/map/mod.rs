extern crate image;

use group::*;
use image::{ImageBuffer, Rgba};
use tools::constant::CONFIRMA;
use node::link::*;
use std::io;
use std::path::Path;
use super::*;

pub mod network;

/// Returns the difference between the lowest and highest x and y values, respectively.
pub fn gen_map_dimensions(min_max: ((i16, i16), (i16, i16))) -> (u32, u32) {
    let x = min_max.0;
    let y = min_max.1;
    ((x.1 - x.0) as u32, (y.1 - y.0) as u32)
}

/// Finds the min and max Nodes and returns ((minX, minY),(maxX, maxY))
pub fn gen_min_max(list: &[Node]) -> ((i16, i16), (i16, i16)) {
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

/// Sets the additions required to center the pixels on the map.
/// This allows for negative placements of Coordinates, when adjusted with this function.
pub fn gen_stuff(min_max: ((i16, i16), (i16, i16))) -> (i16, i16) {

    let x = min_max.0;
    let y = min_max.1;

    (-x.0, -y.0)
}

/// Generates a canvas from the image crate.
pub fn gen_canvas(w: u32, h: u32) -> image::ImageBuffer<Rgba<u8>, Vec<u8>> {
    image::DynamicImage::new_rgba8(w, h).to_rgba()
}

// TODO
pub fn groups_and_links(groups: &[Group], links: &[Link], path: &str) { // TODO implementing.
    // Node size.
    let node_size: u32 = get_node_size_from_groups(&groups);

    // Gets the highest and lowest of all the coordinates.
    let min_max = min_max(groups);

    // Stabilizes the picture to have the action in the center of the image.
    let add = gen_stuff(min_max);

    // Generates an image buffer.
    let mut imgbuf = generate_image_buffer(node_size, min_max);

    for group in groups.iter() {
        group.draw(&mut imgbuf, add.0 as u32, add.1 as u32, node_size);
    }

    for link in links.iter() {
        link.draw_width(&mut imgbuf, add.0 +node_size as i16/2, add.1 +node_size as i16/2, 3);
    }

    let _ = imgbuf.save(&Path::new(path));

}

/// Default implementation for mapping a list of Groups to an image.
pub fn map_groups(path: &Path, groups: &[Group]) {
    // Node size.
    let node_size: u32 = get_node_size_from_groups(&groups);

    // Gets the min and max of the canvas.
    let min_max = min_max(groups);

    // Stabilizes the picture to have the action in the center of the image.
    let add = gen_stuff(min_max);

    // Generates an image buffer.
    let mut imgbuf = generate_image_buffer(node_size, min_max);

    for group in groups.iter() {
        group.draw(&mut imgbuf, add.0 as u32, add.1 as u32, node_size);
    }

    // Save the image to local storage.
    let _ = imgbuf.save(path); // TODO improve

}

/// Generates an image buffer and will ask for confirmation for larger inputs.
pub fn generate_image_buffer(node_size: u32, min_max: ((i16, i16), (i16, i16))) -> image::ImageBuffer<Rgba<u8>, Vec<u8>>  {
    // If there is no image to render. Panic.
    if min_max == ((0,0),(0,0)) {
        panic!("Nothing to map!");
    }

    // Gets the resolution of the image
    let res = gen_map_dimensions(min_max);

    // Sets the image size.
    let width  = res.0 + node_size*2;
    let height = res.1 + node_size*2;

    // Confirm for larger images.
    if CONFIRMA && width+height >= 10000 { // TODO make this disable-able.
        // Confirm the image size before proceeding.
        println!("The image will be {}x{} pixels. Continue? [Y/N]", width, height);
        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(_) => {}
            Err(error) => println!("error: {}", error),
        }
        match input.as_str() {
            "N\n" => panic!("interrupted"),
            _ => (),
        }
    }

    // Create a new ImgBuf with width: imgx and height: imgy
    gen_canvas(width, height)
}

// TODO
pub fn node_and_links(path: &Path, nodes: &[Node], links: &[Link]) {
    // Node size.
    let node_size: u32 = get_node_size(&nodes);

    let min_max = gen_min_max(nodes);

    // Stabilizes the picture to have the action in the center of the image.
    let add = gen_stuff(min_max);

    // Generates an image buffer.
    let mut imgbuf = generate_image_buffer(node_size, min_max);

    // Draws all nodes.
    map_nodes(&mut imgbuf, &nodes, add, node_size);

    // Draws all links
    map_links(&mut imgbuf, &links, add, node_size);

    println!("Mapped: {} nodes & {} links", nodes.len(), links.len());

    // Save the image to local storage.
    let _ = imgbuf.save(path);
}


/// Returns the max Node size existing inside a list of Nodes.
pub fn get_node_size(nodes: &[Node]) -> u32 {
    let mut node_size: u32 = 3; // Minimum default size.
    for node in nodes.iter() {
        let rad = node.radius;
        match rad {
            Some(val) => {if val > node_size {node_size = val;}}
            None => (),
        }
    }
    node_size
}

/// Returns the max Node size existing inside a list of Groups.
pub fn get_node_size_from_groups(groups: &[Group]) -> u32 {
    let mut node_size: u32 = 3; // Minimum default size.
    for group in groups.iter() {
        let tmp = get_node_size(group.get_nodes());
        if tmp > node_size {node_size = tmp;}
    }
    node_size
}

/// Returns a list of Links connecting the Nodes in the order they were provided.
pub fn sequentially_link_nodes(nodes: &[Node]) -> Vec<Link> {
    let mut link_vec = Vec::new();
    for i in 1..nodes.len() {
        let mut link = Link::new(
            &nodes.get(i-1).unwrap().geo,
            &nodes.get(i).unwrap().geo);
        link.color = nodes.get(i).unwrap().color.clone();

        link_vec.push(link);
    }
    link_vec
}

/// Draws the Nodes on an ImageBuffer.
pub fn map_nodes(mut image: &mut ImageBuffer<Rgba<u8>, Vec<u8>>, nodes: &[Node],  add: (i16, i16), node_size: u32) {

    // Iterate over the coordinates and pixels of the image
    for node in nodes {
        node.draw(&mut image, add.0 as u32, add.1 as u32, node_size);
    }
}

/// Draws the Links on an ImageBuffer.
pub fn map_links(mut image: &mut ImageBuffer<Rgba<u8>, Vec<u8>>, links: &[Link], add: (i16, i16), node_size: u32) {

    // Iterate over the coordinates and pixels of the image
    for link in links {
        link.draw(&mut image, add.0 +node_size as i16/2, add.1 +node_size as i16/2);
    }

}
