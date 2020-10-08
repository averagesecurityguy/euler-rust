/*
The sum of the squares of the first ten natural numbers is,

    1^2 + 2^2 + ... + 10^2 = 385

The square of the sum of the first ten natural numbers is,

    (1 + 2 + ... + 10)^2 == 55^2 == 3025

Hence the difference between the sum of the squares of the first ten natural numbers and the square of the sum is,

    3025 - 385 = 2640

Find the difference between the sum of the squares of the first one hundred natural numbers and the square of the sum.
*/

fn main() {
    let mut sum_of_squares: u64 = 0;
    let mut square_of_sums: u64 = 0;
    let difference;

    for x in 1..=100 {
        sum_of_squares += x*x;
        square_of_sums += x;
    }

    square_of_sums *= square_of_sums;
    difference = square_of_sums - sum_of_squares;

    println!("Difference between {} and {} is {}", square_of_sums, sum_of_squares, difference);
}
