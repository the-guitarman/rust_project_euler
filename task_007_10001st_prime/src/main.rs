extern crate termion;
use termion::{clear};
//use termion::{clear, cursor};

//extern crate unicode_segmentation;
//use unicode_segmentation::UnicodeSegmentation;

fn main() {
    println!("{}", clear::All);

    let prime = find_prime(10001);
    println!("10001st prime is: {}", prime);
}

fn find_prime(nth_prime: i64) -> i64 {
    let mut counter = 0;
    let mut number = 1;

    while counter < nth_prime {
        number = number + 1;
        if is_prime(&number) {
            counter = counter + 1;
            println!("prime number {} is: {}", counter, number);
        }
    }

    return number;
}

fn is_prime(number: &i64) -> bool {
    for n in 2..*number {
        if number % n == 0 {
            return false;
        }
    }
    return true;
}

#[test]
fn test_1_find_prime() {
    assert!(find_prime(1) == 2);
    assert!(find_prime(2) == 3);
    assert!(find_prime(3) == 5);
    assert!(find_prime(4) == 7);
    assert!(find_prime(5) == 11);
    assert!(find_prime(6) == 13);
}
