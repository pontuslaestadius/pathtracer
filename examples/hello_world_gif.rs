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
    // Each frame increases time non-linearly.
    let nr_frames = 20;
    let width = 150 + nr_frames*2;
    let height = 150 + nr_frames*2;

    let mut gif_struct = Gif::new("out.gif", width, height).unwrap();
    let color = [
        Rgba {data: [250, 20, 20, 255]},
        Rgba {data: [20, 20, 250, 255]},
        Rgba {data: [20, 250, 20, 255]}
    ];

    for i in 1..nr_frames+1 {
        let mut map_struct = Map::new();
        let i = i as u32;
        let children = i*10;
        let radius = [
            Some(i*2),
            Some(i),
            Some(i/2 +1)
        ];
        let i = i as i16;

        let mut groups = Group::from_list(&[(-i, 0),(i*2, -i*2),(i*2, i*2)]);

        for (j, ref mut group) in groups.iter_mut().enumerate() {
            group.settings.radius = radius[j];
            group.settings.color = color[j];
            map::network::add_children(group, children);
        }

        let mapped_struct = map_struct.map(&groups);
        gif_struct.push_frame(mapped_struct.image.unwrap()).unwrap();
    }
}
