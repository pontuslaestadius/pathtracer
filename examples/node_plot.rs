extern crate gif;
extern crate image;
extern crate pathfinder;
use pathfinder::{map::gif::*, *};

fn main() -> Result<(), std::io::Error> {
    let frames = 10;
    let width = 600;
    let height = 100;
    let radius = 50;

    let x_max = width / radius;
    let y_max = (height / radius);
    let count = x_max * y_max;
    println!("{} groups per row. {} rows. {} total.", x_max, y_max, count);

    let mut gif = Gif::new(width + 8, height + 8);
    let _ = gif.init("out.gif")?;

    let f = |i: usize, d: f64| -> Coordinate {
        let i = i as i16;
        Coordinate {
            x: (d * f64::cos(i as f64)) as i16,
            y: (d * f64::sin(i as f64)) as i16,
        }
    };

    for _ in 1..(frames + 1) {
        let mut groups = Vec::new();

        for c in 0..count {
            let c = c as i16;
            let mut group = Group::new_simple(
                (c % x_max as i16) * radius as i16,
                (c / x_max as i16) * radius as i16,
            );
            group.settings.radius = Some(radius as u32);
            group.settings.color = tools::gen_rgba();
            for _ in (radius as usize /2)..radius as usize {
                for k in 1..(2+c/5) {
                    let f1 = |i: usize| -> Coordinate { f(i, (6*k) as f64) };
                    group.node_plot(&f1);
                }
            }
            groups.push(group);
        }

        let mut map = Map::new();
        map = map.map(&groups);
        gif.push_frame(&map.image.unwrap())?;
    }
    Ok(())
}
