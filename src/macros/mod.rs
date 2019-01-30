#[macro_export]
macro_rules! node {
    () => {
        node!(0, 0);
    };

    ($c:expr) => {
        node!($c.x, $c.y);
    };

    ($x:expr, $y:expr) => {
        node!(&format!("{},{}", $x, $y), $x, $y);
    };

    ($name:expr, $x:expr, $y:expr) => {
        Node::new($name, Coordinate::new($x as i16, $y as i16));
    };
}

#[cfg(test)]
mod tests {
    use super::super::*;

    #[test]
    fn node() {
        let a = node!(0, 0);
        let b = node!("0,0", 0, 0);
        let c = node!();
        let d = node!(Coordinate::new(0, 0));

        assert_eq!(a, b);
        assert_eq!(b, c);
        assert_eq!(c, d);

        let e = node!("not the same!", 0, 0);
        let f = node!(1, 0);
        let g = node!(Coordinate::new(1, 0));

        assert_ne!(a, e);
        assert_ne!(a, f);
        assert_ne!(a, g);
    }
}
