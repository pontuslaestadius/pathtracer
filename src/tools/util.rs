extern crate rand;
use super::constants;
use rand::distributions::{IndependentSample, Range};
use image::Rgba;

// Standard println with an applied condition.
pub fn debug_print(str: &str) {
    if constants::DEBUGMODE {
        println!("{}", str);
    }
}

// Returns a random number between the min and maximum.
pub fn roll(min: u32, max: u32) -> u32 {
    let mut rng = rand::thread_rng();
    let between: Range<u32> = Range::new(min, max);
    between.ind_sample(&mut rng) as u32
}

// Returns a random item from a given list.
pub fn get_random_item(list: &[String]) -> &String {
    let roll = roll(0, list.len() as u32);
    &list[roll as usize]
}

// Checks so that the applied adjustments stay within a u8.
pub fn border(a: u8, b: i32) -> u8 {
    let a = a as i32;

    // If it's too big.
    if a+b > 255 {
        255 as u8
        // If it's too small.
    } else if a+b < 0 {
        0 as u8
        // If it's alright.
    } else {
        (a+b) as u8
    }
}

// Returns a random Rgb color. the opacity is always 255.
pub fn gen_rgba() -> Rgba<u8> {

    // Node
    let mut primary: Rgba<u8> = Rgba {data: [0,0,0,255]};

    // Color of the node.
    for i in 0..4 {
        let v = primary.data[i] as u32 + roll(0,255);

        // If v goes above what a u8 can take. Set it to max.
        let v2 = if v > 255 {
            255
        } else {
            v
        };

        primary.data[i] = v2 as u8;
    }

    primary
}

#[test]
fn test_border() {
    assert_eq!(border(0, 0), 0);
    assert_eq!(border(0, -55), 0);
    assert_eq!(border(0, -255), 0);
    assert_eq!(border(0, 55), 55);
    assert_eq!(border(0, 255), 255);

    assert_eq!(border(255, 0), 255);
    assert_eq!(border(255, -255), 0);
    assert_eq!(border(255, -255), 0);
    assert_eq!(border(100, 100), 200);


}