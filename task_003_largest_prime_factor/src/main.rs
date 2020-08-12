extern crate termion;

use termion::{clear};
//use termion::{clear, cursor};

fn main() {
    println!("{}", clear::All);

    let number = 600716379584; //9227
    //let number = 13195; //29

    let result = find_largest_prime_factor_of(number);
    println!("003: The largest prime factor of {} is: {:?}", number, result);
}

fn find_largest_prime_factor_of(number: i64) -> i64 {
    let (mut number, mut max_divider) = try_to_divide_by_2(number);
    let mut divider = 3;

    while number > 1 {
        if number % divider == 0 {
            while number % divider == 0 {
                number = number / divider;
            }
            max_divider = divider;
        }
        divider = divider + 2;
    }

    max_divider
}

fn try_to_divide_by_2(mut number: i64) -> (i64, i64) {
  let mut max_divider = 1;

  if number % 2 == 0 {
    max_divider = 2;
    while number % 2 == 0 {
        number = number / 2;
    }
  }

  (number as i64, max_divider as i64)
}


#[test]
fn test_01_try_to_divide_by_2() {
    let (number, max_divider) = try_to_divide_by_2(1);
    assert!(number == 1);
    assert!(max_divider == 1);
}

#[test]
fn test_02_try_to_divide_by_2() {
    let (number, max_divider) = try_to_divide_by_2(2);
    assert!(number == 1);
    assert!(max_divider == 2);
}

#[test]
fn test_03_try_to_divide_by_2() {
    let (number, max_divider) = try_to_divide_by_2(3);
    assert!(number == 3);
    assert!(max_divider == 1);
}

#[test]
fn test_04_try_to_divide_by_2() {
    let (number, max_divider) = try_to_divide_by_2(4);
    assert!(number == 1);
    assert!(max_divider == 2);
}

#[test]
fn test_05_try_to_divide_by_2() {
    let (number, max_divider) = try_to_divide_by_2(5);
    assert!(number == 5);
    assert!(max_divider == 1);
}

#[test]
fn test_06_try_to_divide_by_2() {
    let (number, max_divider) = try_to_divide_by_2(6);
    assert!(number == 3);
    assert!(max_divider == 2);
}

#[test]
fn test_07_try_to_divide_by_2() {
    let (number, max_divider) = try_to_divide_by_2(7);
    assert!(number == 7);
    assert!(max_divider == 1);
}

#[test]
fn test_08_try_to_divide_by_2() {
    let (number, max_divider) = try_to_divide_by_2(8);
    assert!(number == 1);
    assert!(max_divider == 2);
}

#[test]
fn test_09_try_to_divide_by_2() {
    let (number, max_divider) = try_to_divide_by_2(9);
    assert!(number == 9);
    assert!(max_divider == 1);
}

#[test]
fn test_10_try_to_divide_by_2() {
    let (number, max_divider) = try_to_divide_by_2(10);
    assert!(number == 5);
    assert!(max_divider == 2);
}

#[test]
fn test_find_largest_prime_factors_of_13195() {
    assert_eq!(find_largest_prime_factor_of(13195), 29);
}

#[test]
fn test_find_largest_prime_factors_of_600716379584() {
    assert_eq!(find_largest_prime_factor_of(600716379584), 9227);
}