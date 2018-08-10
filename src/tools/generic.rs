use std::io;

pub trait Draw {

    fn draw() -> Result<(), io::Error>;

    fn adjust_rgb(&self, r: i8, g: i8, b: i8);

    fn adjust_rgba(&self, r: i8, g: i8, b: i8, a: i8);

}

pub trait Store {

    fn from_str(str: &str) -> Self;

    fn from_string(string: String) -> Self;

}

pub trait Transform {

    fn as_str(&self) -> &str;

    fn as_string(&self) -> String;

}

