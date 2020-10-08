use euler;

/*
A palindromic number reads the same both ways. The largest palindrome made
from the product of two 2-digit numbers is 9009 = 91 × 99.

Find the largest palindrome made from the product of two 3-digit numbers.
*/

fn main() {
    let mut largest = 0;

    for x in (100..1000).rev() {
        for y in (100..1000).rev() {
            if euler::is_palindrome_number(x*y) {
                if x*y > largest { largest = x*y; }
            }
        }
    }

    println!("The largest palindrome is {}", largest);
}
