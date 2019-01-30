use crate::*;

#[test]
fn city() {
    let mut pos = Vec::new();
    let city_size = 30;
    let spread = 15;

    for y in 0..city_size / 2 {
        for x in 0..city_size * 2 {
            let mut node = node!(spread * x as i16, spread * y as i16);
            node.color = tools::seed_rgba((city_size * x + spread * y) as u64);
            pos.push(node);
        }
    }

    pos = Node::linked_list_predicate(pos, &|a, b| {
        let d = (a - b).abs().sum();
        d < spread * 3
    });

    Map::new().map_filter(&pos, &|node: &Node| node.hl(0).is_ok());
}

#[test]
fn git_log() {}

#[test]
fn hello_world_gif() {}

#[test]
fn mvp() -> std::io::Result<()> {
    let pos = [(0, 0), (100, 100), (150, 50), (2000, 4000), (400, 600)];

    let nodes = Node::from_list(&pos);
    let nodes = Node::linked_list(nodes);
    let net = Network::new(nodes);
    let path = net.path("A", "E");
    let rev_path = net.path_rev("E", "A");

    node::path_print(&path?);
    node::path_print(&rev_path?);
    Ok(())
}

#[test]
fn random() {
    let mut groups = Vec::new();
    let coordinates = shape::Circle::new().area(10);
    let children: u32 = 10;
    let radius: u32 = 5;
    let spread = 5;

    for (i, c) in coordinates.iter().enumerate() {
        let mut group = Group::new_simple(c.x * spread, c.y * spread);
        group.radius(radius);
        group.color(tools::seed_rgba((i * 70) as u64));
        group.add(children);
        groups.push(group);
    }
    Map::new().map(&groups);
}

#[test]
fn ellipse() {
    let mut group = Group::new("", Coordinate::new(0, 0));
    group.radius(200);
    group.add(50);
    group.radius(800);
    group.nodes = Node::linked_list(group.nodes);
    group.each(&|node: &mut Node| {
        match node.hl_mut(0) {
            Ok(e) => e.style(2),
            Err(_) => (),
        }
    });
    Map::new().map(&[group]);
}

#[test]
fn cycles() -> std::io::Result<()> {
    let mut gif = map::gif::Gif::new("out.gif", 150, 200);
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
        n.color = image::Rgba([255, 50, 50, 255]);
    }
    gif.cycle(2, balls);

    for _ in 0..10 {
        gif.blank()?;
    }
    Ok(())
}

#[test]
fn graph() {
    let pos = vec![(0, -100), (0, 0), (300, 0)];
    let wrapper = Node::linked_list(Node::from_list(&pos));

    let mut pos = Vec::new();
    let y = vec![5, 30, 45, 35, 40, 80, 75, 70, 25, 30];
    let spread = 300 / (y.len() - 1) as i16;
    for (i, y) in y.iter().enumerate() {
        pos.push((i as i16 * spread, -*y));
    }
    let line = Node::linked_list(Node::from_list(&pos));

    Map::new().map(&wrapper).map(&line);
}

#[test]
fn hello_world() {}

#[test]
fn node_plot() {}

#[test]
fn types() {}
