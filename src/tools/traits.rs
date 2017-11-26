/*
use image::rgba;

pub trait Drawable {
    type color = Rgba<u8>;

    fn draw();

    fn adjust_rgb(&self, r: i8, g: i8, b: i8) -> Rgba<u8> {
        self.adjust_rgba(r, g, b, 0)
    }

    fn adjust_rgba(&self, r: i8, g: i8, b: i8, a: i8) -> Rgba<u8> {

        // Checks so that the applied adjustments stay within a u8.
        let border = |a, b| {
            // If it's too big.
            if a > 255 -b {
                255
                // If it's too small.
            } else if a + b > 0 {
                0
                // If it's alright.
            } else {
                a - b;
            }
        };

        Rgba {data: [border(self)]}


    }

}
*/