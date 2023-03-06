/*
n! means n × (n − 1) × ... × 3 × 2 × 1

For example, 10! = 10 × 9 × ... × 3 × 2 × 1 = 3628800,
and the sum of the digits in the number 10! is 3 + 6 + 2 + 8 + 8 + 0 + 0 = 27.

Find the sum of the digits in the number 100!
*/

fn main() {
    let mut sum = 0;
    let digits = euler::bigints::factorial(100);

    for (_, c) in digits.chars().enumerate() {
        sum = sum + c.to_digit(10).unwrap();
    }

    println!("Digits: {}", digits);
    println!("Sum of digits {}", sum);
}
