extern crate image;

use std::fs::File;
use std::path::Path;


pub fn test_map() {

    let imgx = 300;
    let imgy = 300;

    // Create a new ImgBuf with width: imgx and height: imgy
    let mut imgbuf = image::ImageBuffer::new(imgx, imgy);

    // Iterate over the coordinates and pixels of the image
    for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
        if x % 2 == 0 {
            *pixel = image::rgb([0,0,255]);
        }
    }

    // Save the image as “fractal.png”
    let ref mut fout = File::create(&Path::new("map.png")).unwrap();

    // We must indicate the image’s color type and what format to save as
    let _ = image::ImageLuma8(imgbuf).save(fout, image::PNG);
}