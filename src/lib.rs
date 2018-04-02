#![cfg_attr(feature="clippy", feature(plugin))]
#![cfg_attr(feature="clippy", plugin(clippy))]

extern crate image;
extern crate rand;

pub mod node;
pub mod map;
pub mod tools;
pub mod group;
pub mod data;

pub mod tests;
