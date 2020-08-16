extern crate termion;
use termion::{clear};
//use termion::{clear, cursor};

extern crate unicode_segmentation;
use unicode_segmentation::UnicodeSegmentation;

fn main() {
    println!("{}", clear::All);

    let palindrom = find_largest_palindrome_product();
    println!("Largest palindrome product is: {}", palindrom);
}

fn find_largest_palindrome_product() -> i64 {
    let mut result = 0;

    for x in (100..1000).rev() {
        for y in 100..1000 {
            if x <= y {
                break;
            }

            let product = x * y;
            if is_palindrome(product) && product > result {
                result = product;
                println!("Palindrome found: {} * {} = {}", x, y, result);
            }
        }
    }
    
    return result;
}

fn is_palindrome(number: i64) -> bool {
    let mut string1 = number.to_string();
    let string2 = number.to_string();

    return 
        if string1.graphemes(true).count() == 1 {
            true
        } else {
            let (first, _last) = string1.split_at_mut(string2.graphemes(true).count() / 2);

            let word: &str = first;
            let drow: String = word
                // Split the string into an Iterator of &strs, where each element is an
                // extended grapheme cluster.
                .graphemes(true)
                // Reverse the order of the grapheme iterator.
                .rev()
                // Collect all the chars into a new owned String.
                .collect();

            string1.ends_with(&drow)
        }
}

#[test]
fn test_1_is_palindrome() {
    assert!(is_palindrome(1));
}

#[test]
fn test_2_is_palindrome() {
    assert!(is_palindrome(9009));
}

#[test]
fn test_3_is_palindrome() {
    assert!(is_palindrome(90209));
}

#[test]
fn test_4_is_palindrome() {
    assert!(is_palindrome(1234) == false);
}

#[test]
fn test_5_is_palindrome() {
    assert!(is_palindrome(12345) == false);
}

#[test]
fn test_6_find_largest_palindrome_product() {
    assert!(find_largest_palindrome_product() == 906609);
}