extern crate pathfinder;

use pathfinder::{Coordinate, Square, Node, Network, Draw};

fn main() {

    let a: Node<Square> = Node::new("A", Coordinate::new(0,0));
    let mut b: Node<Square> = Node::new("B", Coordinate::new(100,100));
    let mut c: Node<Square> = Node::new("C", Coordinate::new(150,50));
    let mut d: Node<Square> = Node::new("D", Coordinate::new(100,0));

    b.link(&a);
    c.link(&b);
    d.link(&c);

    // let map = Map::new();
    // let map = map.map(&[d.clone(), c.clone(), b.clone(), a.clone()]blu);

    //let path= std::path::Path::new("mvp.png");
    //let _ = map.image.unwrap().save(&path);

    let nodes: Vec<Node<Square>> = [d.clone(), c.clone(), b.clone(), a.clone()].to_vec();
    let net = Network::new(nodes);
    let path = net.path("D", "A", &Network::path_shortest_leg);
    print_path(&path);
}

fn print_path<T: Draw>(path: &Vec<(usize, &T)>) {
    let mut distance = 0;
    let mut prev = Coordinate::new(0,0);
    for &(link_i, leg) in path.iter() {
        let dis = pathfinder::node::coordinates::distance(&prev, &leg.get_coordinate());
        distance += dis;
        prev = leg.get_coordinate().clone();
        println!("#{} ({:?}) - {}px", link_i, leg.get_coordinate(), dis);
    }

    println!("Total distance: {}", distance);
}