use euler;

/*
By listing the first six prime numbers: 2, 3, 5, 7, 11, and 13, we can see that the 6th prime is 13.

What is the 10 001st prime number?
*/

fn main() {
    println!("The 10 001st prime is {}", euler::nth_prime_number(10001));
}
