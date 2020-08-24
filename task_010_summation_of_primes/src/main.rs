extern crate termion;
use termion::{clear};
//use termion::{clear, cursor};

//extern crate unicode_segmentation;
//use unicode_segmentation::UnicodeSegmentation;

fn main() {
    println!("{}", clear::All);

    let sum = find_summation_of_primes(2_000_000);

    println!("Summation of primes is: {}", sum);
}

fn find_summation_of_primes(max_number: i64) -> i64 {
    let mut sum = 0;
    for i in 2..max_number {
        if is_prime(&i) {
            sum = sum + i;
            println!("{} is a prime", i);
        } else {
            println!("{} is no prime", i);
        }
    }
    return sum;
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
fn test_1_find_summation_of_primes() {
    assert!(find_summation_of_primes(10) == 17);
}
