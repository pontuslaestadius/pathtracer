
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
        Node::new(String::new(), Coordinate::new(x2, y1)),
        Node::new(String::new(), Coordinate::new(x1, y2)),
        Node::new(String::new(), Coordinate::new(x2, y2))
    ]
}

pub fn cube(start_point: &Coordinate, width: i16, height: i16, depth_height: i16, depth_width: i16) -> Vec<Node> {

    let rem =
        if depth_width < 0 {
            if depth_height < 0 {
                3
            } else {
                1
            }
    } else {
        if depth_height < 0 {
            2
        } else {
            0
        }
    };


    cube_precise(
        start_point.x,
        start_point.y,
        start_point.x + width,
        start_point.y + height,
        depth_height,
        depth_width,
        rem
    )
}

fn cube_precise(x1: i16, y1: i16, x2: i16, y2: i16, dh: i16, dw: i16, rem: usize) -> Vec<Node> {
    let mut first = rectangle_precise(x1, y1, x2, y2);
    let mut second = rectangle_precise(x1+dw, y1+dh, x2+dw, y2+dh);

    second.remove(rem);
    first.append(&mut second);

    first
}


/// Given a list of nodes, will return all pixels inside the formation they have.
fn get_pixels_in_area(list: &[Node]) -> Vec<Coordinate> {
    Vec::new()
}
