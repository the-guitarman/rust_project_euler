extern crate termion;
use termion::{clear};
//use termion::{clear, cursor};

//extern crate unicode_segmentation;
//use unicode_segmentation::UnicodeSegmentation;

fn main() {
    println!("{}", clear::All);

    let numbers = find_highly_divisible_triangular_number(500);

    println!("Highly divisible triangular number is: {}", numbers);
}

fn find_highly_divisible_triangular_number(number_of_dividers: usize) -> u64 {
    let mut sum = 1;
    let mut number = 1;

    loop {
        number = number + 1;

        sum = sum + number;
        let dividers = dividers_count(sum);

        println!("T{} = {}, dividers: {}", number, sum, dividers);

        if dividers > number_of_dividers {
            break;
        }
    }

    return sum;
}

fn dividers_count(number: u64) -> usize {
    let mut count = 0;
    for i in (1..(number + 1)).rev() {
        if number % i == 0 {
            count = count + 1;
        }
    }
    return count;
}

#[test]
fn test_1_find_highly_divisible_triangular_number() {
    assert!(find_highly_divisible_triangular_number(5) == 28);
}
