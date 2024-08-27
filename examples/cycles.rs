extern crate gif;
extern crate image;
extern crate pathtracer;
extern crate rand;

use pathtracer::{map::gif::*, *};
use std::env;

fn main() -> std::io::Result<()> {
    let mut gif = Gif::new("out.gif", 150, 200);
    let mut balls = Node::from_list(&[
        (50, 55),
        (70, 70),
        (40, 70),
        (49, 78),
        (27, 82),
        (15, 92),
        (30, 71),
        (60, 100),
        (50, 92),
        (25, 85),
    ]);

    for n in balls.iter_mut() {
        n.color = image::Rgb([255, 50, 50]);
    }

    gif.cycle(2, balls);
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        panic!("Can't plant nodes without config file.");
    }
    let nodes = Node::from_file(&args[1])?;
    let mut nodes = Node::linked_list(nodes);
    for n in nodes.iter_mut() {
        n.color = image::Rgb([0, 100, 0]);
    }

    gif.cycle(1, nodes);
    for _ in 0..10 {
        gif.blank().unwrap();
    }
    Ok(())
}
