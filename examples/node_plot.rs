extern crate gif;
extern crate image;
extern crate pathfinder;
use pathfinder::{map::gif::*, *};

fn main() -> std::io::Result<()> {
    let frames = 4;
    let width = 290;
    let height = 90;
    let radius = 38;
    let x_max: i16 = (width / radius) as i16;
    let count: i16 = x_max * (height / radius) as i16;
    let mut gif = Gif::new("out.gif", width, height + 5);

    let f = |i: usize, d: f64| -> Coordinate {
        let i = i as f64;
        Coordinate {
            x: (d * f64::cos(i)) as i16,
            y: (d * f64::sin(i)) as i16,
        }
    };

    let mut groups = Vec::new();
    for c in 0i16..count {
        let rad = radius as i16;
        let mut group = cluster!((c % x_max) * rad, (c / x_max) * rad);
        group.radius(radius as u32);
        group.color(tools::seed_rgb(c as u64 * 32));

        for _ in 5..radius as usize {
            for k in 1..(4 + c / 5) {
                group.node_plot(&|i: usize| -> Coordinate { f(i, 4.5 * k as f64) });
            }
        }
        groups.push(group);
    }

    for _ in 0..frames {
        for g in groups.iter_mut() {
            g.rotate(20.0);
        }

        gif.push(Map::new().map(&groups))?;
    }
    Ok(())
}
