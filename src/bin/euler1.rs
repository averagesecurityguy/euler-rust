use euler;

/*
If we list all the natural numbers below 10 that are multiples of 3 or 5, we
get 3, 5, 6 and 9. The sum of these multiples is 23.

Find the sum of all the multiples of 3 or 5 below 1000.
*/

fn main() {
    let mut sum = 0;

    for n in 1..1000 {
        if euler::is_multiple(n, 3) || euler::is_multiple(n, 5) {
            sum += n;
        }
    }

    println!("The sum is {}", sum);
}
