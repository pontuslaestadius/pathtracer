use std::io;
use gif;
use gif::*;
use image::{Rgba, ImageBuffer};
use std::fs::File;
use super::super::Map;

pub struct Gif  {
    pub encoder: gif::Encoder<File>,
    pub width: u16,
    pub height: u16,
}

impl Gif {
    /// Constructs a Gif struct and initializes a file on the system for the Gif to be stored.
    pub fn new(output: &str, width: u16, height: u16) -> Result<Gif, io::Error> {
        // Initalize encoder.
        let file = File::create(output)?;
        let mut encoder = gif::Encoder::new(file, width, height, &[])?;
        encoder.set(Repeat::Infinite)?;
        Ok(Gif {
            encoder,
            width,
            height
        })
    }

    /// Pushes a frame using a map struct.
    pub fn push_map(&mut self, map: Map) -> Result<(), io::Error> {
        self.push_frame(&map.image.unwrap())
    }

    /// Pushes a frame to the Gif structure, This also immediately saves it to disk.
    pub fn push_frame(&mut self, image: &ImageBuffer<Rgba<u8>, Vec<u8>>) -> Result<(), io::Error> {
        let mut pixels: Vec<u8> = Vec::new();

        for pix in image.pixels() {
            pixels.push(pix.data[0]);
            pixels.push(pix.data[1]);
            pixels.push(pix.data[2]);
            pixels.push(pix.data[3]);
        }

        // Create frame from data
        let mut frame = gif::Frame::from_rgba(image.width() as u16, image.height() as u16, &mut pixels);
        frame.dispose = DisposalMethod::Background;
        self.encoder.write_frame(&frame)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_gif_new() {
        let gif = Gif::new("test_gif_new.gif", 50, 50);
        assert_eq!(gif.is_ok(), true);
        let gif = gif.unwrap();
        assert_eq!(gif.width == 50 && gif.height == 50, true);
        let _ = fs::remove_file("test_gif_new.gif").unwrap();
    }

    #[test]
    fn test_push_frame() {

    }

}

