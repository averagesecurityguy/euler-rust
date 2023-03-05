use euler;

/*
2520 is the smallest number that can be divided by each of the numbers from 1
to 10 without any remainder. What is the smallest positive number that is
evenly divisible by all of the numbers from 1 to 20?
*/

fn main() {
    let n: u64 = 20;
    let mut product: u64 = 1;

    for x in euler::primes::primes_below_n(n) {
        let mut val: u64 = x;

        while val <= n {
            val *= x;
        }

        product = product * (val / x)
    }

    println!("Product is {}", product);
}
