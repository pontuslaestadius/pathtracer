extern crate image;

use std::fs::File;
use std::path::Path;
use std::f64;

use super::tools::constants;
use self::util::debug_print;
use pathfinder::node::Node;
use pathfinder::node::nodelink::NodeLink;

use super::tools::util;

use pathfinder::map::image::GenericImage;

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
    let node_size = 20;

    let min_max = gen_min_max(list);

    let res = gen_map_dimensions(min_max);
    let add = gen_stabalize(min_max);

    // Sets the imag
    let imgx = (res.1 +2) as u32;
    let imgy = (res.0 +2) as u32;

    println!("Creating Nodemap with resolution: {}x{}", imgx, imgy);

    // Create a new ImgBuf with width: imgx and height: imgy
    let mut imgbuf = image::ImageBuffer::new(imgx, imgy);
    let luma = image::Luma([200 as u8]);
    let luma_background = image::Luma([50 as u8]);

    // Counts the number of nodes placed.
    let mut placed_nodes = 0; // TODO this wont be required once the map is 100% functioning.

    // Adds background nodes first.
    let range = util::roll((list.len()/2) as u32, list.len() as u32);

    for _ in 0..range {
        let roll_x = util::roll(0, imgx);
        let roll_y = util::roll(0, imgy);
        imgbuf.put_pixel(roll_x, roll_y, luma_background);
    }

    // Iterate over the coordiantes and pixels of the image
    for node in list {
        // println!("x: node.geo.x: {} add_x: {} y: node.geo.y: {} add_y: {}", node.geo.x, add_x, node.geo.y, add_y);
        let x = ((node.geo.x + add.0) as i16); // TODO can overflow
        let y = (node.geo.y + add.1) as i16; // TODO can overflow

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
    let mut node_size: u32 = 6;
    let node_name_length: u32 = 100;

    let min_max = gen_min_max(nodes);

    let res = gen_map_dimensions(min_max);
    let add = gen_stabalize(min_max);

    // Sets the imag
    let imgx = (res.0 + node_size*2 +node_name_length) as u32;
    let imgy = (res.1 + node_size) as u32;

    println!("Creating map_node_and_links with resolution: {}x{}", imgx, imgy);

    // Create a new ImgBuf with width: imgx and height: imgy
    let mut imgbuf = image::ImageBuffer::new(imgx, imgy);

    // Counts the number of nodes placed.
    let mut placed_links = 0; // TODO this wont be required once the map is 100% functioning.
    let mut placed_nodes = 0; // TODO this wont be required once the map is 100% functioning.

    let scale = |a: i16, b: i16| {
        let mut res: f64 = 0.0;
        if b != 0 {
            res = (a.abs() as f64 / b.abs() as f64);
        }
        res
    };

    // Iterate over the coordinates and pixels of the image
    for link in links {
        let from = link.from;
        let to = link.to;

        let from_x = (from.geo.x + add.0 + (node_size) as i16) as i16;
        let from_y = (from.geo.y + add.1 + (node_size) as i16) as i16;

        let to_x = (to.geo.x + add.0 + (node_size) as i16) as i16;
        let to_y = (to.geo.y + add.1 + (node_size) as i16) as i16;

        let dif_x = from_x - to_x;
        let dif_y = from_y - to_y;

        let scale_y: f64 = scale(dif_x, dif_y);
        let scale_x: f64 = scale(dif_y, dif_x);

        let mut x = from_x;
        let mut y = from_y;

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

        let luma_link = image::Luma([util::roll(50,60) as u8]);
        let luma_link2 = image::Luma([util::roll(15,20) as u8]);

        let end = |a, b, c| {
            if a == b {
                return 0
            }
            c
        };

        while x != to_x || y != to_y {
            x_ite += scale_x;
            y_ite += scale_y;

            inc_x = end(x, to_x, inc_x);
            inc_y = end(y, to_y, inc_y);

            if          x_ite >= 1.0 && y_ite >= 1.0 {
                if x_ite > y_ite {
                    x += inc_x;
                    x_ite -= 1.0;
                } else {
                    y += inc_y;
                    y_ite -= 1.0;
                }
            } else if   y_ite >= 1.0 {
                y += inc_y;
                y_ite -= 1.0;
            } else if   x_ite >= 1.0 {
                x += inc_x;
                x_ite -= 1.0;
            } else {
                println!("edge case error: ({}, {})", inc_x, inc_y); // TODO this occurs from time to time.
                break;
            }

            let xa: u32 = x as u32;
            let ya: u32 = y as u32;

            imgbuf.blend_pixel(xa, ya+1, luma_link2);
            imgbuf.blend_pixel(xa, ya, luma_link);
        }
        placed_links += 1;
    }

    // Iterate over the coordinates and pixels of the image
    for node in nodes {
        let luma_node = image::Luma([util::roll(150,160) as u8]);
        let luma_node2 = image::Luma([util::roll(170,180) as u8]);

        // println!("x: node.geo.x: {} add_x: {} y: node.geo.y: {} add_y: {}", node.geo.x, add_x, node.geo.y, add_y);
        let x = ((node.geo.x + add.0) as i16); // TODO can overflow
        let y = (node.geo.y + add.1) as i16; // TODO can overflow

        let range = 100;
        let inc = f64::consts::PI/(range/2) as f64;
        for i in 0..range +1 {
            let a: f64 = f64::consts::PI * (inc * (i -range/2) as f64);

            let r = node_size as f64;

            // gets a point on the circle's circumference.
            let cir = |a: f64, b: f64| a + r * b;

            let inc_x = cir(x as f64, a.cos()) as i16;            // x = cx + r * cos(a)
            let inc_y = cir(y as f64, a.sin()) as i16;            // y = cy + r * sin(a)

            if (inc_x < 0) {
                break;
            }

            let mut inc_luma = 0;
            {
                let mut p = imgbuf.get_pixel(x as u32, y as u32);
                inc_luma = (p.data[0]/2) as u8;
            }
            let base: u8 = 70 +(i as f64 *0.4) as u8;
            if inc_luma + base > 250 {
                inc_luma = 240 - base;
            }
            // println!("luma: {}", base+inc_luma);
            let luma_this = image::Luma([base+inc_luma as u8]);
            let luma_that = image::Luma([((base+inc_luma)/2) as u8]);
            let luma_that2 = image::Luma([((base+inc_luma)/2 +3) as u8]);
            let luma_that_less = image::Luma([((base+inc_luma)/2 -12) as u8]);

            imgbuf.put_pixel((inc_x +node_size as i16 +1) as u32, (inc_y +node_size as i16 +1) as u32, luma_that_less);
            imgbuf.put_pixel((inc_x +node_size as i16) as u32, (inc_y +node_size as i16 +1) as u32, luma_that);
            imgbuf.put_pixel((inc_x +node_size as i16 +1) as u32, (inc_y +node_size as i16) as u32, luma_that2);
            imgbuf.put_pixel((inc_x +node_size as i16) as u32, (inc_y +node_size as i16) as u32, luma_this);
        }

        /*
        for i in 0..node_size +1 {
            for j in 0..node_size +1 {
                if (i == 0 || i == node_size || j == node_size || j == 0) {
                    imgbuf.blend_pixel((x+(i + node_size/2) as i16) as u32, (y+(j + node_size/2) as i16) as u32, luma_node);
                } else {
                    imgbuf.blend_pixel((x+(i + node_size/2) as i16) as u32, (y+(j + node_size/2) as i16) as u32, luma_node_center);
                }
            }
        }
        */

        placed_nodes += 1;
    }

    // Save the image
    let fout = &mut File::create(&Path::new("resources/network_map.png")).unwrap();

    if constants::DEBUGMODE {
        println!("Placed: {} Nodes", placed_nodes);
        println!("Placed: {} Links", placed_links);
    }

    // We must indicate the image’s color type and what format to save as
    let _    = image::ImageLuma8(imgbuf).save(fout, image::PNG);
}

/*

fn gen_pixel_name(string: String) -> [u8] {
    let mut vec: Vec<[u8]> = Vec::new();
    for (_, c) in string.chars().enumerate() {
        vec.push(gen_pixel_letter(c));
    }
    vec
}

fn gen_pixel_letter(letter: char) -> [u8] {
    // TODO read from a file, instead of writing it in source code.

    // Draws an a.
    [   1, 1, 1, 1,
        0, 0, 0, 1,
        1, 1, 1, 1,
        1, 0, 0, 1,
        1, 1, 1, 1
    ]
}
*/