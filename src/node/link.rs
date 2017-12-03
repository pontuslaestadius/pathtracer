use super::coordinates::Coordinates;
use image::{ImageBuffer, Rgba};

use super::util::gen_rgba;

/*
     Link
     --------
     Holds connections between two struct with coordinates.
 */

pub struct Link<'a> {
    pub from: &'a Coordinates,
    pub to: &'a Coordinates,
}

impl<'a> Link<'a> {

    pub fn draw(&self, image: &mut ImageBuffer<Rgba<u8>, Vec<u8>>, x_offset: i16, y_offset: i16, size: u32) {

        //let pixel: Rgba<u8> = Rgba {data: [0,0,0,255]};
        let pixel = gen_rgba();

        let a = (size/2) as i16;

        let mut x = self.from.x +x_offset +a;
        let mut y = self.from.y +y_offset +a;

        let mut x_to = self.to.x +x_offset +a;
        let mut y_to = self.to.y +y_offset +a;

        let mut move_x = x -x_to;
        let mut move_y = y -y_to;

        println!("{},{} -> {},{} | {}±{}", x, y, x_to, y_to, move_x, move_y);

        let mov = | mut rep: i16, mut start: i16| {
            let mut save: Vec<i16> = Vec::new();
            while rep != 0 {
                save.push(start);

                let i = if rep <0 {
                    1
                } else if       rep >0 {
                    -1
                } else {
                    0
                };
                start += i;
                rep += i;
            }
            save
        };

        let mut x_ = mov(move_x, x);
        let mut y_ = mov(move_y, y);

        x = self.from.x +x_offset;
        y = self.from.y +y_offset;

        let mut pos = Vec::new();

        let scale: f64 = x_.len() as f64 /(y_.len()+1) as f64;
        let mut c: f64 = scale;

        while !x_.is_empty() || !y_.is_empty() {
            //println!("c: {},  {},{}", c, x, y);
            pos.push(Coordinates::new(x, y));

            if c <= 0.00 && !x_.is_empty() && !y_.is_empty() {
                x = x_.remove(0);
                y = y_.remove(0);
                c += scale;
            } else {
                if x_.len() > y_.len()  {
                    x = x_.remove(0);
                } else  /*x_.len() < y_.len()*/{
                    y = y_.remove(0);
                }
                c-=1.00;
            }

        }
        pos.iter().map(|c|
                           {
                               image.put_pixel( c.x  as u32, c.y as u32, pixel)
                           } ).collect::<Vec<_>>();

    }

    /// Creates a new Link and binds two nodes together.
    pub fn new(from: &'a Coordinates, to: &'a Coordinates) -> Link<'a> {
        Link {
            from,
            to,
        }
    }
}

impl<'a> PartialEq for Link<'a> {
    fn eq(&self, other: &Link) -> bool {
        (self.from == other.from) &&
            (self.to == other.to)
    }
}
