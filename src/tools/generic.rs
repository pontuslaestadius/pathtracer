use std::io;
// TODO either use or remove.
pub trait Draw {

    fn draw() -> Result<(), io::Error>;

    fn adjust_rgb(&self, r: i8, g: i8, b: i8) {
        self.adjust_rgba(r, g, b, 0)
    }

    fn adjust_rgba(&self, r: i8, g: i8, b: i8, a: i8);
    /*
    {
        self.Color = Rgba {data: [
            border(Color.data.0, r),
            border(Color.data.1, g),
            border(Color.data.2, b),
            border(Color.data.3, a)
        ]}
    }
    */
}

pub trait Store {
    fn from_str(str: &str) -> Self;

    fn from_string(string: String) -> Self;
}

pub trait Transform {
    fn as_str(&self) -> &str;

    fn as_string(&self) -> String;
}