extern crate gif;
extern crate image;
extern crate pathfinder;
use pathfinder::{map::gif::*, *};

fn main() -> Result<(), std::io::Error> {
    let frames = 10;
    let width = 600;
    let height = 100;
    let radius = 50;
    let x_max: i16 = (width / radius) as i16;
    let count = x_max * (height / radius) as i16;
    let mut gif = Gif::new(width + 8, height + 8);
    let _ = gif.init("out.gif")?;

    let f = |i: usize, d: f64| -> Coordinate {
        let i = i as f64;
        Coordinate {
            x: (d * f64::cos(i)) as i16,
            y: (d * f64::sin(i)) as i16,
        }
    };

    for frame_nr in 1..(frames + 1) {
        let mut groups = Vec::new();

        for c in 0..count {
            let c = c as i16;
            let mut group =
                Group::new_simple((c % x_max) * radius as i16, (c / x_max) * radius as i16);
            group.settings.radius = Some(radius as u32);
            group.settings.color = tools::seed_rgba(c as u64 * 40000);

            for _ in (radius as usize / 2)..radius as usize {
                for k in 1..(2 + c / 5) {
                    let f1 = |i: usize| -> Coordinate { f(i, (6 * k) as f64) };
                    group.node_plot(&f1);
                }
            }
            group.rotate(f64::from(frame_nr * (360 / frames)));
            groups.push(group);
        }

        let mut map = Map::new();
        map = map.map(&groups);
        gif.push_frame(&map.image.unwrap())?;
    }
    Ok(())
}
