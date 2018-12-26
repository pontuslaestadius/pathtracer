extern crate gif;
extern crate image;
extern crate pathfinder;
use pathfinder::{map::gif::*, *};

fn main() -> std::io::Result<()> {
    let frames = 6;
    let width = 300;
    let height = 90;
    let radius = 38;
    let x_max: i16 = (width / radius) as i16;
    let frame_rot = f64::from(90 / (frames - 1));
    let count = x_max * (height / radius) as i16;
    let mut gif = Gif::new("out.gif", width, height + 10);

    let f = |i: usize, d: f64| -> Coordinate {
        let i = i as f64;
        Coordinate {
            x: (d * f64::cos(i)) as i16,
            y: (d * f64::sin(i)) as i16,
        }
    };

    let mut groups = Vec::new();
    for c in 0..count {
        let c = c as i16;
        let mut group = Group::new_simple((c % x_max) * radius as i16, (c / x_max) * radius as i16);
        group.radius(radius as u32);
        group.color(tools::seed_rgba(c as u64 * 32));

        for _ in 5..radius as usize {
            for k in 1..(4 + c / 5) {
                let f1 = |i: usize| -> Coordinate { f(i, 4.5 * k as f64) };
                group.node_plot(&f1);
            }
        }
        groups.push(group);
    }

    // For each frame, we rotate the nodes in the group.
    // Always creates a seemless infinite spin.
    for _ in 1..(frames + 1) {
        for g in groups.iter_mut() {
            g.rotate(frame_rot);
        }

        gif.push(Map::new().map(&groups))?;
    }
    Ok(())
}
