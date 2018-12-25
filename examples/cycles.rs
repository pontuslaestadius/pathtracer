extern crate gif;
extern crate image;
extern crate pathfinder;
extern crate rand;

use pathfinder::{map::gif::*, *};
use std::env;

fn main() -> std::io::Result<()> {
    let mut gif = Gif::new(150, 200);
    let _ = gif.init("out.gif")?;
    let mut balls = Node::from_list(&[
        (50, 55),
        (70, 70),
        (40, 70),
        (49, 78),
        (37, 62),
        (27, 82),
        (15, 92),
        (30, 71),
        (60, 100),
        (50, 92),
    ]);

    for n in balls.iter_mut() {
        n.color = image::Rgba([255, 50, 50, 255]);
    }

    gif.add_cycle(2, balls);
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        panic!("Can't plant nodes without config file.");
    }
    let nodes = Node::from_file(&args[1])?;
    let mut nodes = Node::linked_list(nodes);
    for n in nodes.iter_mut() {
        n.color = image::Rgba([0, 100, 0, 255]);
    }

    for _ in 0..10 {
        gif.push_map(Map::new().map(&nodes))?
    }
    Ok(())
}
