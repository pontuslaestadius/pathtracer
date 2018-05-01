use super::{Group, Shape};
use std::cmp::{min, max};

/// Counts the amount of Groups and child Nodes.
pub fn count<T: Shape>(list: &[Group<T>]) -> (usize, usize) {
    let mut n: usize = 0;
    for g in list.iter() {
        n+=g.nodes.len();
    }
    (list.len(), n)
}

/// Finds the min and max nodes used for scaling and setting the boundaries of an image.
pub fn min_max<T: Shape>(list: &[Group<T>]) -> ((i16, i16), (i16, i16)) {
    let mut mi = (0, 0);
    let mut ma = (0, 0);

    for group in list {
        for node in group.nodes.iter() {
            // Iterates over the nodes and finds the minimum and maximum x and y values.
            ma.0 = max(node.geo.x, ma.0);
            ma.1 = max(node.geo.y, ma.1);

            mi.0 = min(node.geo.x, mi.0);
            mi.1 = min(node.geo.y, mi.0);

        }
    }

    ((mi.0, ma.0), (mi.1, ma.1))
}
