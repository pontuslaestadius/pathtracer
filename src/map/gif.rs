

use std::io;
use gif;
use gif::*;
use image::{Rgba, ImageBuffer};
use std::io::Write;
use std::fs::File;
use image;


pub struct Gif  {
    pub encoder: gif::Encoder<File>,
    pub width: u16,
    pub height: u16,
}

impl Gif {
    pub fn new(output: &str, width: u16, height: u16) -> Result<Gif, io::Error> {
        // Initalize encoder.
        let mut file = File::create(output).unwrap();
        let mut encoder = gif::Encoder::new(file, width, height, &[])?;
        encoder.set(Repeat::Infinite)?;
        Ok(Gif {
            encoder: encoder,
            width,
            height
        })
    }

    pub fn push_frame(&mut self, mut image: ImageBuffer<Rgba<u8>, Vec<u8>>) -> Result<(), io::Error> {

        let mut pixels: Vec<u8> = Vec::new();

        for pix in image.pixels() {
            pixels.push(pix.data[0]);
            pixels.push(pix.data[1]);
            pixels.push(pix.data[2]);
            pixels.push(pix.data[3]);
        }

        // Create frame from data
        let mut frame = gif::Frame::from_rgba(image.width() as u16, image.height() as u16, &mut pixels);
        //let mut frame = image::Frame::new(image); // this would be nice. Damn piston devs!!!
        frame.dispose = DisposalMethod::Background;
        self.encoder.write_frame(&frame)?;

        Ok(())
    }
}