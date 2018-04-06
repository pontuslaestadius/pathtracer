use super::Group;

/// Counts the amount of Groups and child Nodes.
pub fn count(list: &[Group]) -> (usize, usize) {
    let mut n: usize = 0;
    for g in list.iter() {
        n+=g.nodes.len();
    }
    (list.len(), n)
}

/// Finds the min and max nodes used for scaling and setting the boundaries of an image.
pub fn min_max(list: &[Group]) -> ((i16, i16), (i16, i16)) {
    let mut min_x = 0;
    let mut min_y = 0;
    let mut max_x = 0;
    let mut max_y = 0;

    for group in list {
        for node in group.nodes.iter() {
            // Iterates over the nodes and finds the minimum and maximum x and y values.
            if node.geo.x > max_x {
                max_x = node.geo.x;
            }
            if min_x > node.geo.x {
                min_x = node.geo.x;
            }

            if node.geo.y > max_y {
                max_y = node.geo.y;
            }
            if min_y > node.geo.y {
                min_y = node.geo.y;
            }
        }
    }

    ((min_x, max_x), (min_y, max_y))
}
