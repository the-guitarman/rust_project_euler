extern crate termion;
use termion::{clear};
//use termion::{clear, cursor};

//extern crate unicode_segmentation;
//use unicode_segmentation::UnicodeSegmentation;

fn main() {
    println!("{}", clear::All);

    let numbers = find_x();

    println!("X is: {}", numbers);
}

fn find_x() -> u64 {
    return 0;
}

#[test]
fn test_1_() {
    assert!(true);
}
