extern crate gif;
extern crate image;
extern crate pathfinder;
extern crate rand;

use image::Rgba;
use pathfinder::{map::gif::*, *};

fn main() -> std::io::Result<()> {
    let mut gif = Gif::new(200, 100);
    let _ = gif.init("out.gif")?;
    let radius = [30, 20, 40];
    let color = [[250, 20, 20, 255], [20, 20, 250, 255], [20, 250, 20, 255]];

    for _ in 0..10 {
        let mut groups = Group::from_list(&[(0, 0), (45, 40), (110, 20)]);

        for (j, ref mut group) in groups.iter_mut().enumerate() {
            group.radius(radius[j]);
            group.color(Rgba { data: color[j] });
            map::network::add_children(group, 100);
        }

        let mut map = Map::new();
        map = map.map(&groups);
        gif.push_frame(&map.image.unwrap())?
    }
    Ok(())
}
