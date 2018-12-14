use super::super::{Map, IW};
use gif::{self, *};
use std::{fs::File, io};

pub struct Gif {
    pub encoder: Option<gif::Encoder<File>>,
    pub width: u16,
    pub height: u16,
}

impl Gif {
    /// Constructs a Gif struct and initializes a file on the system for the
    /// Gif to be stored.
    pub fn new(width: u16, height: u16) -> Self {
        Gif {
            encoder: None,
            width,
            height,
        }
    }

    /// Initializes image encoder and creates output file.
    pub fn init(&mut self, output: &str) -> Result<(), io::Error> {
        let file = File::create(output)?;
        let mut encoder = Encoder::new(file, self.width, self.height, &[])?;
        encoder.set(Repeat::Infinite)?;
        let _ = self.encoder.get_or_insert(encoder);
        Ok(())
    }

    /// Pushes a frame using a map struct.
    pub fn push_map(&mut self, map: Map) -> Result<(), io::Error> {
        self.push_frame(&map.consume())
    }

    /// Pushes a frame to the structure, This also immediately saves it to disk.
    pub fn push_frame(&mut self, image: &IW) -> Result<(), io::Error> {
        let mut pixels: Vec<u8> = Vec::new();
        for pix in image.image().pixels() {
            for i in 0..4 {
                pixels.push(pix.data[i]);
            }
        }
        let dim = image.dimensions();
        let mut frame = Frame::from_rgba(dim.x as u16, dim.y as u16, &mut pixels);
        frame.dispose = DisposalMethod::Background;
        frame.delay = 20;
        let mut e = self.encoder.take().unwrap();
        e.write_frame(&frame)?;
        let _ = self.encoder.get_or_insert(e);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_gif_new() {
        let mut gif = Gif::new(50, 50);
        let output = "test_gif_new.gif";
        let init_res = gif.init(output);
        assert!(init_res.is_ok());
        assert!(gif.width == 50 && gif.height == 50);
        let _ = fs::remove_file(output).unwrap();
    }

}
