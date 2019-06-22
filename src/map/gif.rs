/*!
  Gif wrapper for managing moving images.

  Applied logic for cycles of Nodes.
*/

use super::super::*;
use gif::{self, *};
use std::{fs::File, io};

struct Cycle<'a, T: Draw + Location + Hash + MinMax + Copy> {
    interval: u8,
    count: u8,
    map: Vec<T>,
    predicate: &'a Fn(&T) -> T,
}

impl<'a, T: Draw + Location + Hash + MinMax + Copy> Cycle<'a, T> {
    pub fn new(interval: u8, map: Vec<T>, predicate: &'a Fn(&T) -> T) -> Self {
        Cycle {
            interval,
            map,
            predicate,
            count: interval,
        }
    }

    pub fn then(&mut self) -> Option<Vec<T>> {
        self.count -= 1;
        if self.count == 0 {
            self.count = self.interval;
            self.map = self.map.iter().map(self.predicate).collect::<_>();
            Some(self.map.clone())
        } else {
            None
        }
    }
}

/**
Wrapper around the Gif struct found in the gif crate.

With additions for pathfinder support, and cycling features.
*/
pub struct Gif<'a> {
    encoder: gif::Encoder<File>,
    cycles: Vec<Cycle<'a, Node>>,
    frames: u16,
    width: u16,
    height: u16,
}

impl<'a> Gif<'a> {
    /**
    Constructs a Gif struct and initializes a file on the system for the Gif to be stored.


    ## Errors

    If it is unable to create the Encoder, or if the file can not be created.


    ## See also

    examples/hello_world_gif.rs
    example/cycles.rs

    */
    pub fn new(filename: &str, width: u16, height: u16) -> Self {
        let file = File::create(filename).unwrap();
        let mut encoder = Encoder::new(file, width, height, &[]).unwrap();
        encoder.set(Repeat::Infinite).unwrap();
        Gif {
            encoder,
            cycles: Vec::new(),
            frames: 0,
            width,
            height,
        }
    }

    /**
    Adds in a repeating patttern every interval frame on to the gif image.


    ## See also

    examples/cycles.rs
    */
    pub fn cycle(&mut self, interval: u8, map: Vec<Node>) {
        self.cycles.push(Cycle::new(interval, map, &|x| *x));
    }

    /**
    Adds in a repeating patttern every interval frame on to the gif image.
    */
    pub fn cycle_predicate(
        &mut self,
        interval: u8,
        map: Vec<Node>,
        predicate: &'a Fn(&Node) -> Node,
    ) {
        self.cycles.push(Cycle::new(interval, map, predicate));
    }

    /**
    Removes all cycles from the gif.
    */
    pub fn remove_cycles(&mut self) {
        self.cycles = Vec::new();
    }

    /**
    Advances the cycles and returns the patterns it matched.
    */
    pub fn advance_cycle(&mut self) -> Vec<Node> {
        let mut result = Vec::new();
        for cycle in self.cycles.iter_mut() {
            if let Some(mut e) = cycle.then() {
                result.append(&mut e)
            }
        }
        result
    }

    /**
    Returns the number of frames that has been written.
    */
    pub fn frames(&self) -> u16 {
        self.frames
    }

    /**
    Pushes a frame using a map struct.
    */
    pub fn push(&mut self, mut map: Map) -> Result<(), io::Error> {
        map = map.map(&self.advance_cycle());
        self.push_frame(&map.consume())
    }

    /**
    Pushes a frame to the structure, This also immediately saves it to disk.

    ## Errors

    If the encoder fails to write the frame to disk.
    */
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

    /**
    Appends a blank frame to the gif.

    This will also advance Gif cycles.
    */
    pub fn blank(&mut self) -> Result<(), io::Error> {
        let mut node = node!(self.width as i16 - 1, self.height as i16 - 1);
        node.radius = Some(0);
        self.push(Map::new().map(&[node]))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn define(case: &Fn(Gif) -> std::io::Result<()>) {
        let gif = Gif::new("/tmp/test_gif_new.gif", 50, 50);
        case(gif).unwrap();
        // This can break Travis CI. Because the file doesn't get created?
        //let _ = std::fs::remove_file("test_gif_new.gif").unwrap();
    }

    #[test]
    fn test_gif_new() {
        define(&|_| Ok(()));
    }

    #[test]
    fn blank_frames() {
        define(&|mut gif| {
            assert_eq!(gif.frames(), 0);
            gif.blank()?;
            assert_eq!(gif.frames(), 1);
            Ok(())
        });
    }

    #[test]
    fn cycles_predicate() {
        define(&|mut gif| {
            gif.cycle_predicate(1, vec![node!(25, 25)], &|x| {
                let mut x = x.clone();
                x.geo.x += 5;
                x
            });

            for i in 0..3 {
                assert_eq!(gif.advance_cycle()[0].x(), 30 + i * 5);
            }

            Ok(())
        });
    }

    #[test]
    fn cycles_every_frame() {
        define(&|mut gif| {
            gif.cycle(1, vec![node!(25, 25)]);
            for _ in 0..3 {
                assert_eq!(gif.advance_cycle().len(), 1);
            }
            Ok(())
        });
    }

    #[test]
    fn cycles_every_other() {
        define(&|mut gif| {
            gif.cycle(2, vec![node!(25, 25)]);
            assert!(gif.advance_cycle().is_empty());
            assert_eq!(gif.advance_cycle().len(), 1);
            Ok(())
        });
    }
}
