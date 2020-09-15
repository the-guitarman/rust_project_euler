extern crate termion;
use termion::{clear};
//use termion::{clear, cursor};

extern crate num;
use num::bigint::BigUint;

//extern crate unicode_segmentation;
//use unicode_segmentation::UnicodeSegmentation;

const RADIX: u32 = 10;

fn main() {
    println!("{}", clear::All);

    let numbers = find_power_digit_sum(1000);
    println!("power digit sum is: {}", numbers);
}

fn find_power_digit_sum(exponent: u32) -> u32 {
    BigUint::new(vec![2])
        .pow(exponent)
        .to_string()
        .chars()
        .map(|char| char.to_digit(RADIX).unwrap())
        .sum::<u32>()
}

#[test]
fn test_1_find_power_digit_sum() {
    assert!(find_power_digit_sum(15) == 26);
}
