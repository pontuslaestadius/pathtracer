extern crate rand;
use super::constants;
use rand::distributions::{IndependentSample, Range};

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