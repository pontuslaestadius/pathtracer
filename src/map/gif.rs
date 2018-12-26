use super::super::*;
use gif::{self, *};
use std::{fs::File, io};

struct Cycle<T: Draw + Location + Hash + MinMax + Copy> {
    interval: u8,
    count: u8,
    map: Vec<T>
}

impl<T: Draw + Location + Hash + MinMax + Copy> Cycle<T> {
    pub fn new(interval: u8, map: Vec<T>) -> Self {
        Cycle {
            interval,
            map,
            count: interval,
        }
    }

    pub fn then(&mut self) -> Option<Vec<T>> {
        self.count -= 1;
        if self.count == 0 {
            self.count = self.interval;
            Some(self.map.clone())
        } else {
            None
        }
    }
}

pub struct Gif {
    encoder: gif::Encoder<File>,
    cycles: Vec<Cycle<Node>>,
    frames: u16,
}

impl Gif {
    /// Constructs a Gif struct and initializes a file on the system for the
    /// Gif to be stored.
    pub fn new(filename: &str, width: u16, height: u16) -> Self {
        let file = File::create(filename).unwrap();
        let mut encoder = Encoder::new(file, width, height, &[]).unwrap();
        encoder.set(Repeat::Infinite).unwrap();
        Gif {
            encoder,
            cycles: Vec::new(),
            frames: 0
        }
    }

    /// Adds in a repeating patttern every interval frame on to the gif image.
    pub fn add_cycle(&mut self, interval: u8, map: Vec<Node>) {
        self.cycles.push(Cycle::new(interval, map));
    }

    /// Removes all cycles from the gif.
    pub fn remove_cycles(&mut self) {
        self.cycles = Vec::new();
    }

    /// Advances the cycles and returns the patterns it matched.
    pub fn advance_cycle(&mut self) -> Vec<Node> {
        let mut result = Vec::new();
        for cycle in self.cycles.iter_mut() {
            match cycle.then() {
                Some(mut e) => result.append(&mut e),
                None => (),
            }
        }
        result
    }

    /// Returns the number of frames that has been written.
    pub fn frames(&self) -> u16 {
        self.frames
    }

    /// Pushes a frame using a map struct.
    pub fn push(&mut self, mut map: Map) -> Result<(), io::Error> {
        map = map.map(&self.advance_cycle());
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
        self.encoder.write_frame(&frame)?;
        self.frames += 1;
        Ok(())
    }

    /// Appends a blank frame to the gif.
    pub fn blank(&mut self) -> Result<(), io::Error> {
        let mut node = Node::new("", Coordinate::new(0, 0));
        node.radius = Some(0);
        self.push(Map::new().map(&[node]))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gif_new() {
        let gif = Gif::new("test_gif_new.gif", 50, 50);
        let _ = std::fs::remove_file("test_gif_new.gif").unwrap();
    }

    #[test]
    fn blank_frames() {
        let mut gif = Gif::new("test_gif_new.gif", 50, 50);
        assert_eq!(gif.frames(), 0);
        gif.blank();
        assert_eq!(gif.frames(), 1);
        let _ = std::fs::remove_file("test_gif_new.gif").unwrap();
    }
}
