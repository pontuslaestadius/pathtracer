extern crate gif;
extern crate image;
extern crate pathfinder;
extern crate rand;

use image::Rgb;
use pathfinder::{map::gif::*, *};

fn main() -> std::io::Result<()> {
    let mut gif = Gif::new("out.gif", 200, 100);
    let radius = [30, 20, 40];
    let color = [[250, 20, 20], [20, 20, 250], [20, 250, 20]];

    for _ in 0..10 {
        let mut groups = Group::from_list(&[(0, 0), (45, 40), (110, 20)]);

        for (j, ref mut group) in groups.iter_mut().enumerate() {
            group.radius(radius[j]);
            group.color(Rgb { data: color[j] });
            group.add(100);
        }

        gif.push(Map::new().map(&groups))?
    }
    Ok(())
}
