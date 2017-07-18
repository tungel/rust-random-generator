extern crate rand;
use rand::distributions::{IndependentSample, Range};

extern crate chrono;
use chrono::prelude::*;

/// Generate random number given a range
/// Note that the range is inclusive, so the generated result can be equal to
/// min or max
fn get_random_inclusive(min: &u8, max: &u8) -> u8 {
    // https://doc.rust-lang.org/rand/rand/trait.Rng.html#method.gen_range
    // Note: should not use Rng `gen_range`
    // This is a convenience wrapper around distributions::Range.
    // If this function will be called repeatedly with the same arguments,
    // one should use Range, as that will amortize the computations that allow
    // for perfect uniformity, as they only happen on initialization.

    let between = Range::new(*min, *max + 1);
    let mut rng = rand::thread_rng();
    between.ind_sample(&mut rng)
}

fn main() {
    println!("----------------------------------------------------------");
    println!("{}", Local::now());

    // range, inclusive
    let min: u8 = 1;
    let max: u8 = 45;
    let mut result: Vec<u8> = Vec::new();

    for _ in 0..7 {
        let mut random_num = get_random_inclusive(&min, &max);
        while result.iter().any(|e| e == &random_num) {
            random_num = get_random_inclusive(&min, &max);
        }

        result.push(random_num);
        print!("{}\t", random_num);
    }
    println!();
    // println!("vec len = {}", result.len())
}
