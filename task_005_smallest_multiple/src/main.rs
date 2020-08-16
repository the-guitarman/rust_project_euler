extern crate termion;
use termion::{clear};
//use termion::{clear, cursor};

//extern crate unicode_segmentation;
//use unicode_segmentation::UnicodeSegmentation;

fn main() {
    println!("{}", clear::All);

    let multiple = find_smallest_multiple(20);
    println!("Smallest multiple is: {}", multiple);
}

fn find_smallest_multiple(up_to: i64) -> i64 {
    let mut number = up_to;
    let mut found = false;

    while found == false {
        for x in 1..(up_to + 1) {
            if x == up_to && number % x == 0  {
                found = true;
            } else if number % x != 0 {
                break;
            }
        }

        if found == false {
            number = number + 1;
        }
    }

    return number
}

#[test]
fn test_1_find_smallest_multiple() {
    assert!(find_smallest_multiple(10) == 2520);
}
