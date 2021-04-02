/*!
Global constants

These constants are used to set default values for certain properties.
 */

// A Node cannot have more connections than this.
pub const MAX_LINKS: usize = 5;

// Used as a hashing dividor.
pub const NETWORK_REM: usize = 666;

// Default node size (width and height).
pub const DEFAULT_SIZE: u16 = 4;

// Default node shader.
pub const DEFAULT_SHADE: u16 = 20;

// Node link Weight.
pub const DEFAULT_LINK_SIZE: u16 = 2;

// Default color for everything.
pub const DEFAULT_RGBA: image::Rgba<u8> = image::Rgba {
    data: [0, 0, 0, 255],
};
