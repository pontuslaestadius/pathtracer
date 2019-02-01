
/**
  initalize Coordinates using a range of parameters.


  ## Examples

  These are all equal.

  ```
  # #![macro_use] use pathfinder::*;
  # fn main() {

  coordinate!();
  coordinate!(0);
  coordinate!(0, 0);

  # }
  ```
*/
#[macro_export]
macro_rules! coordinate {
    () => {
        coordinate!(0, 0);
    };

    ($c:expr) => {
        coordinate!($c, $c);
    };

    ($x:expr, $y:expr) => {
        Coordinate::new($x as i16, $y as i16);
    };
}

/**
  initalize Nodes using a range of parameters.


  ## Examples

  These are all equal.

  ```
  # #![macro_use] use pathfinder::*;
  # fn main() {

  node!();
  node!(coordinate!());
  node!(0, 0);
  node!("0,0", 0, 0);

  # }
  ```
*/
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
        Node::new($name, coordinate!($x, $y));
    };
}

/**
  initalize Groups using a range of parameters.


  ## Examples

  These are all equal.

  ```
  # #![macro_use] use pathfinder::*;
  # fn main() {

  cluster!();
  cluster!(coordinate!());
  cluster!(0, 0);
  cluster!("0,0", 0, 0);

  # }
  ```
*/
#[macro_export]
macro_rules! cluster {
    () => {
        cluster!(0, 0);
    };

    ($c:expr) => {
        cluster!($c.x, $c.y);
    };

    ($x:expr, $y:expr) => {
        cluster!(&format!("{},{}", $x, $y), $x, $y);
    };

    ($name:expr, $x:expr, $y:expr) => {
        Group::new($name, coordinate!($x, $y));
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

    #[test]
    fn node_any_type() {
        let _ = node!(0u64, 0.5 as f64);
        let _ = node!(0u32, 4000);
        let _ = node!(0u16, 9u8);
        let _ = node!(0u8, 0i32);
        let _ = node!(0f64, 100);
    }

    #[test]
    fn cluster() {
        let a = cluster!(0, 0);
        let b = cluster!("0,0", 0, 0);
        let c = cluster!();
        let d = cluster!(Coordinate::new(0, 0));

        assert_eq!(a, b);
        assert_eq!(b, c);
        assert_eq!(c, d);

        let e = cluster!("not the same!", 0, 0);
        let f = cluster!(1, 0);
        let g = cluster!(Coordinate::new(1, 0));

        assert_ne!(a, e);
        assert_ne!(a, f);
        assert_ne!(a, g);
    }

    #[test]
    fn cluster_any_type() {
        let _ = cluster!(0u64, 0.5 as f64);
        let _ = cluster!(0u32, 4000);
        let _ = cluster!(0u16, 9u8);
        let _ = cluster!(0u8, 0i32);
        let _ = cluster!(0f64, 100);
    }

    #[test]
    fn coordinate() {
        let a = coordinate!(0, 0);
        let b = coordinate!(0);
        let c = coordinate!();

        assert_eq!(a, b);
        assert_eq!(b, c);

        let c = coordinate!(1, 0);

        assert_ne!(a, c);
    }

    #[test]
    fn coordinate_any_type() {
        let _ = coordinate!(0u64, 0.5 as f64);
        let _ = coordinate!(0u32, 4000);
        let _ = coordinate!(0u16, 9u8);
        let _ = coordinate!(0u8, 0i32);
        let _ = coordinate!(0f64, 100);
    }

}
