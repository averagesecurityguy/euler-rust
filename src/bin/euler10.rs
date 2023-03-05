use euler;

/*
The sum of the primes below 10 is 2 + 3 + 5 + 7 = 17.

Find the sum of all the primes below two million.
*/

fn main() {
    let mut sum: u64 = 0;
    
    for p in euler::primes::primes_below_n(2000000) {
        sum += p;
    }

    println!("The sum of the primes below 2000000 is {}", sum);
}
