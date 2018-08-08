extern crate pathfinder;

use pathfinder::*;

fn main() {

    let pos = [
        (0,0),
        (100,100),
        (150,50),
        (100,0)
    ];

    let nodes = Node::from_list(&pos);
    //let nodes = Node::linked_list(nodes);

    let net = Network::new(nodes);
    let path = net.path("D", "A", &Network::path_shortest_leg);
    print_path(&path);
}

fn print_path<T: Draw + Location>(path: &Vec<(usize, &T)>) {
    let mut distance = 0;
    let mut prev = Coordinate::new(0,0);
    for &(link_i, leg) in path.iter() {
        let dis = pathfinder::node::coordinates::distance(prev, leg.get_coordinate());
        distance += dis;
        prev = leg.get_coordinate().clone();
        println!("#{} ({:?}) - {}px", link_i, leg.get_coordinate(), dis);
    }

    println!("Total distance: {}", distance);
}
