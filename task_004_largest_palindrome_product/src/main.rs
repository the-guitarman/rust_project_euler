extern crate termion;

use termion::{clear};
//use termion::{clear, cursor};

fn main() {
    println!("{}", clear::All);
    println!("Largest palindrome product");
}

fn is_palindrome(&_number: i64) -> bool {
    return false
}