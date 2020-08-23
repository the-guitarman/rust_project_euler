extern crate termion;
use termion::{clear};
//use termion::{clear, cursor};

//extern crate unicode_segmentation;
//use unicode_segmentation::UnicodeSegmentation;

fn main() {
    println!("{}", clear::All);

    let numbers = find_special_pythagorean_triplet_product();

    println!("product of special pythagorean triplet is: {}", numbers);
}

fn find_special_pythagorean_triplet_product() -> i64 {
    let mut a = 0;
    let mut b = 0;

    'a: for i in 1..999 {
        for j in 1..999 {
            let diff = 1000 - (i + j);
            if diff > 0 {
                if (square(i) + square(j)) == square(diff) {
                    a = i;
                    b = j;
                    break 'a;
                }
            }
        }
    }

    let c = 1000 - a - b;

    return (a * b * c) as i64;
}

fn square(x: i64) -> u64 {
    return (x * x) as u64;
}

#[test]
fn test_1_() {
    assert!(find_special_pythagorean_triplet_product() == 31875000);
}
