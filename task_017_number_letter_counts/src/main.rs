extern crate termion;
use termion::{clear};
//use termion::{clear, cursor};

//extern crate unicode_segmentation;
//use unicode_segmentation::UnicodeSegmentation;

extern crate numbers;

extern crate regex;
use regex::Regex;

fn main() {
    println!("{}", clear::All);

    let up_to = 1000;
    let result = find_number_letter_counts(up_to);
    println!("Number letter counts of 1 up to {} is: {}", up_to, result);
}

fn find_number_letter_counts(up_to: i64) -> i64 {
    let mut result = 0;
    let re = Regex::new(r"[^a-zA-Z]").unwrap();

    for i in 1..(up_to + 1) {
        result += re
            .replace_all(&numbers::convert(numbers::Language::English, i), "")
            .chars()
            .count();
    }

    return result as i64;
}

#[test]
fn test_1_find_number_letter_counts() {
    assert!(find_number_letter_counts(5) == 19);
}
