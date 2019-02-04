/*!
Global constants

These constants are used to set default values for certain properties.
 */

pub const MAX_LINKS: usize = 5;
pub const NETWORK_REM: usize = 666;
pub const DEFAULT_SIZE: u16 = 4;
pub const DEFAULT_SHADE: u16 = 20;
pub const DEFAULT_LINK_SIZE: u16 = 2;
pub const DEFAULT_RGBA: image::Rgba<u8> = image::Rgba {
    data: [0, 0, 0, 255],
};
