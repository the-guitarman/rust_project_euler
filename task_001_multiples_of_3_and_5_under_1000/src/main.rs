fn main() {
    let border = 1000;
    let result = sum_of_all_multiples_of_3_and_5_under_border(border);
    println!("001: Sum of all multiples of 3 and 5 under {}: {}", border, result);
}

fn sum_of_all_multiples_of_3_and_5_under_border(border: i32) -> i32 {
    let mut result = vec![0; 0];

    for zahl in 1..border {
        if zahl % 3 == 0 || zahl % 5 == 0 {
            result.push(zahl);
        }
    }

    result.iter().sum()
}

#[test]
fn test_sum_of_all_multiples_of_3_and_5_under_1000() {
    assert_eq!(sum_of_all_multiples_of_3_and_5_under_border(1000), 233168);
}
