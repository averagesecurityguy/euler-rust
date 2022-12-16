/*
2^15 = 32768 and the sum of its digits is 3 + 2 + 7 + 6 + 8 = 26.

What is the sum of the digits of the number 2^1000?
*/

fn main() {
    let mut sum = 0;
    let digits = euler::bigints::big_pow(2, 1000);

    for (_, c) in digits.chars().enumerate() {
        sum = sum + c.to_digit(10).unwrap();
    }

    println!("Sum of digits {}", sum);
}
