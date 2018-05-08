extern crate pathfinder;
extern crate rand;
extern crate image;
use pathfinder::node::figure;
use pathfinder::*;
use pathfinder::map::*;

fn main() {

    let mut nodes: Vec<Node<Square>> = Vec::new();

    for i in 0..33   {
        let dif = 10;
        let start = Coordinate::new(-(dif/2)*i,-(dif/2)*i);
        let mut new_nodes = figure::rectangle(&start, dif*i, dif*i);
        nodes.append(&mut new_nodes)
    }

    let start = Coordinate::new(0,0);
    nodes = figure::rectangle(&start, 50, 50);

    //let links = sequentially_link_nodes(&nodes);

    let path = std::path::Path::new("figure.png");

    let mut map = Map::new();
    map = map
        .map(&nodes);
        //.map(&links);
    let _ = map.image.unwrap().save(&path);
}


