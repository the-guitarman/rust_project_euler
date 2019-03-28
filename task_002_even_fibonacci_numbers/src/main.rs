fn main() {
    let border = 4_000_000;
    let sum = sum_of_all_even_fibonacci_numbers_below_border(border);
    println!("002: Sum of all even fibonacci numbers below {}: {}", border, sum);
}

fn sum_of_all_even_fibonacci_numbers_below_border(border: i32) -> i32 {
    let mut number1 = 1;
    let mut number2 = 2;
    let mut next_number = 0;
    let mut sum = number2;

    while next_number < border {
        next_number = number1 + number2;
        number1 = number2;
        number2 = next_number;
        if next_number & 1 == 0 {
            sum += next_number;
        }
    }

    sum
}

#[test]
fn test_sum_of_all_even_fibonacci_numbers_below_4_000_000() {
    assert_eq!(sum_of_all_even_fibonacci_numbers_below_border(4_000_000), 4613732);   
}
