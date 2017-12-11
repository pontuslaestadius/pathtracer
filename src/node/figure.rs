
use super::*;
use super::coordinates::Coordinate;


pub fn rectangle(start_point: &Coordinate, width: i16, height: i16) -> Vec<Node> {
    rectangle_precise(
        start_point.x,
        start_point.y,
        start_point.x +width,
        start_point.y +height,
    )
}


/// Returns 4 nodes in a Vec to form a rectangle.
fn rectangle_precise(x1: i16, y1: i16, x2: i16, y2: i16) -> Vec<Node> {
    vec![
        Node::new(String::new(), Coordinate::new(x1, y1)),
        Node::new(String::new(), Coordinate::new(x1, y2)),
        Node::new(String::new(), Coordinate::new(x2, y1)),
        Node::new(String::new(), Coordinate::new(x2, y2))
    ]
}