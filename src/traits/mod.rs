/*!
Collection of traits.
*/

use super::*;

/**
Functions required to draw the structure on the image.
 */
pub trait Draw {
    fn draw(&self, image: IW, offset: Coordinate, shape: &Shape) -> IW;
    fn size(&self) -> u32;
    fn links(&self) -> &[HL];
}

/**
Enables the structure to be located by X or Y.
 */
pub trait Location {
    /**
    Retrieves the position Coordinates.
     */
    fn position(&self) -> Coordinate;

    /**
    Returns if the positions are equal or not.
     */
    fn eq<L: Location>(&self, other: &L) -> bool { self.position() == other.position() }

    /**
    Retrieves the X coordinate.
     */
    fn x(&self) -> i16 { self.position().x }

    /**
    Retrieves the Y coordinate.
     */
    fn y(&self) -> i16 { self.position().y }

    /**
    Returns the sum of the x and y value.
     */
    fn sum(&self) -> i16 { self.x() + self.y() }
}

/**
Enables retrieving the minimum and maximum position for the structure.
 */
pub trait MinMax {
    fn min_max(&self) -> (Coordinate, Coordinate);
}

/**
Makes it possible to find connected nodes in networks.
 */
pub trait Find: Hash + Location {
    /**
    Matches the Hashes and returns Some if it matches.
     */
    fn find<H: Hash>(&self, hash: H) -> Option<Coordinate> {
        if self.hash() == hash.hash() {
            return Some(self.position());
        }
        None
    }
}

pub trait Hash {
    fn hash(&self) -> u64;
}
