//! Author: Pontus Laestadius
//! Version: 0.3
//! Since: 2017-04-06
//!
//! Visualizes a moving gif.
//!

extern crate pathfinder;
extern crate rand;
extern crate image;
extern crate gif;

use pathfinder::map::gif::*;
use pathfinder::*;
use image::Rgba;

fn main() {

    let width = 300;
    let height = 300;

    let mut gif_struct = Gif::new("git_struct.gif", width, height).unwrap();

    let nr_frames = 100;

    for i in 1..nr_frames+1 {
        println!("Frames: {}", i);
        let mut map_struct = Map::new();

        // ----------------------------------------------

        let i = i as i16;
        let mut group1 = Group::new_simple(-i,0);
        let mut group2 = Group::new_simple(i*2,-i*2);
        let mut group3 = Group::new_simple(i*2,i*2);
        let i = i as u32;

        group1.settings.color = Rgba {data: [250, 20, 20, 255]};
        group2.settings.color = Rgba {data: [20, 20, 250, 255]};
        group3.settings.color = Rgba {data: [20, 200, 20, 255]};

        group1.settings.radius = Some(i*2);
        group2.settings.radius = Some(i);
        group3.settings.radius = Some(i/2 +1);

        // List of groups.
        let mut groups: Vec<Group> = vec!(group1, group2, group3);

        let children = i*10;

        // Add children.
        for group in groups.iter_mut() {
            map::network::add_children(group, children);
        }

        // ----------------------------------------------

        let mapped_struct = map_struct.map(&groups);
        gif_struct.push_frame(mapped_struct.image.unwrap()).unwrap();

    }


}