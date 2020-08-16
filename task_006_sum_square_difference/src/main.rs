extern crate termion;
use termion::{clear};
//use termion::{clear, cursor};

//extern crate unicode_segmentation;
//use unicode_segmentation::UnicodeSegmentation;

fn main() {
    println!("{}", clear::All);

    let difference = find_sum_square_difference(100);
    println!("Sum square difference is: {}", difference);
}

fn find_sum_square_difference(up_to: i64) -> i64 {
    return square_of_the_sum(up_to) - sum_of_the_squares(up_to);
}

fn sum_of_the_squares(up_to: i64) -> i64 {
    let mut sum = 0;
    for x in 1..(up_to + 1) {
        sum = sum + x * x;
    }
    return sum;
}

fn square_of_the_sum(up_to: i64) -> i64 {
    let mut sum = 0;
    for x in 1..(up_to + 1) {
        sum = sum + x;
    }
    return sum * sum;
}

#[test]
fn test_1_find_sum_square_difference() {
    assert!(find_sum_square_difference(10) == 2640);
}
