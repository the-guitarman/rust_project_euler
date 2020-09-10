extern crate termion;
use termion::{clear};
//use termion::{clear, cursor};

//extern crate unicode_segmentation;
//use unicode_segmentation::UnicodeSegmentation;

struct Chain {
    start: u64,
    length: usize,
    elements: Vec<u64>,
}

impl Chain {
    fn new(start: u64) -> Self {
        Self {
            start: start,
            length: 1,
            elements: vec![start],
        }
    }

    fn get_last(&mut self) -> u64 {
        if self.elements.iter().count() == 0 {
            self.elements.push(self.start);
            self.start
        } else {
            *self.elements.last().unwrap()
        }
    }

    fn calc_next(&mut self) {
        let n: u64 = self.get_last();
        let next =
            if n % 2 == 0 {
                n / 2
            } else {
                3 * n + 1
            };
        self.elements.push(next);
        self.length = self.elements.len();
    }

    fn calc_total(&mut self) {
        while self.get_last() > 1 {
            self.calc_next();
        }
    }
}

fn main() {
    println!("{}", clear::All);

    let numbers = find_longest_collatz_sequence(999_999);

    println!("X is: {}", numbers);
}

fn find_longest_collatz_sequence(max_start_number: u64) -> u64 {
    let mut start_number: u64 = 0;
    let mut max_length: usize = 0;
    for start in (1..(max_start_number + 1)).rev() {
        let mut chain = Chain::new(start);
        chain.calc_total();
        if chain.length > max_length {
            max_length = chain.length;
            start_number = start;
        }
    }
    start_number as u64
}

#[test]
fn test_1_find_longest_collatz_sequence() {
    let mut chain = Chain::new(13);
    chain.calc_total();
    assert!(chain.length == 10);
}
