extern crate termion;
use termion::{clear};
//use termion::{clear, cursor};

//extern crate unicode_segmentation;
//use unicode_segmentation::UnicodeSegmentation;

fn main() {
    println!("{}", clear::All);

    let numbers = find_lattice_paths(20);
    println!("Lattice paths of 20 is: {}", numbers);
}

fn find_lattice_paths(length_of_square: u64) -> u64 {
    let mut result: u64 = 1;
    for i in 1..(length_of_square + 1) {
        result = result * (length_of_square + i) / i;
    }
    return result;
}

#[test]
fn test_1_find_lattice_paths() {
    assert!(find_lattice_paths(2) == 6);
}
