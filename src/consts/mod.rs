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
// FIXME: rename to RGB now that we don't have to use an RGBA base.
// Previously that was the only way to unlock certain features for png/jpg.
pub const DEFAULT_RGBA: image::Rgb<u8> = image::Rgb([0, 0, 0]);
