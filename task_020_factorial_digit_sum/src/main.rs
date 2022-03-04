extern crate num_bigint;
use num_bigint::BigInt;
use num_bigint::ToBigInt;

extern crate termion;
use termion::{clear};
//use termion::{clear, cursor};

//use std::char;

fn main() {
    println!("{}", clear::All);

    let sum = find_factorial_digit_sum(100);

    println!("Factorial digit sum is: {}", sum);
}

fn find_factorial_digit_sum(number: u32) -> u32 {
    let factorial = factorial(number);
    
    let mut result = 0;
    for c in factorial.to_str_radix(10).chars() {
        result += c.to_string().parse::<u32>().unwrap();
    }
    result
}

fn factorial(number: u32) -> BigInt {
    let mut result = 1_i32.to_bigint().unwrap();
    for n in 2..(number + 1) {
        result *= n.to_bigint().unwrap();
    }
    result
}

#[test]
fn test_1_() {
    assert!(true);
}

#[test]
fn test_factorial() {
    assert!(find_factorial_digit_sum(10) == 27);
    assert!(find_factorial_digit_sum(100) == 648);
}
