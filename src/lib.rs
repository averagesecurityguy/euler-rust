pub mod fibonacci;
pub mod triangle;
pub mod collatz;

use primes::{Sieve, PrimeSet};

pub fn is_palindrome_number(n: u64) -> bool {
    let s = n.to_string();
    let rev = s.chars().rev().collect::<String>();

    s == rev
}

pub fn is_multiple(n: u64, d: u64) -> bool {
    n % d == 0
}

pub fn factor(composite: u64) -> Vec<u64> {
    return primes::factors(composite);
}

pub fn divisors(composite: u64) -> Vec<u64> {
    let c = composite as f64;
    let sqrt = c.sqrt().floor() as u64;
    let mut divisors = vec![];

    if composite == 1 { divisors.push(1); return divisors; }

    for f in 1..sqrt+1 {
        if composite % f == 0 {
            divisors.push(f);
            divisors.push(composite/f);
        }
    }

    divisors.sort();
    divisors
}

pub fn primes_below_n(n: u64) -> Vec<u64> {
    let mut primes = vec![];
    let mut pset = Sieve::new();

    for (_, prime) in pset.iter().enumerate() {
        if prime >= n { break; }
        primes.push(prime);
    }

    primes
}

pub fn nth_prime_number(n: usize) -> u64 {
    let mut pset = Sieve::new();
    let mut nth: u64 = 1;

    for (_, prime) in pset.iter().enumerate().skip(n-1).take(1) { nth = prime; }

    nth
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_multiple() {
        assert!(!is_multiple(3, 2));
        assert!(is_multiple(4, 2));
    }

    #[test]
    fn test_factor() {
        let factors28 = vec![2, 2, 7];
        let factors30 = vec![2, 3, 5];

        assert_eq!(factor(28), factors28);
        assert_eq!(factor(30), factors30);
    }

    #[test]
    fn test_divisors() {
        let divisors1 = vec![1];
        let divisors3 = vec![1, 3];
        let divisors6 = vec![1, 2, 3, 6];

        assert_eq!(divisors(1), divisors1);
        assert_eq!(divisors(3), divisors3);
        assert_eq!(divisors(6), divisors6);
    }

    #[test]
    fn test_is_palindrome_number() {
        assert!(is_palindrome_number(9009));
        assert!(!is_palindrome_number(9008));
    }

    #[test]
    fn test_primes_below_n() {
        let primes_below_10 = vec![2, 3, 5, 7];
        let primes_below_20 = vec![2, 3, 5, 7, 11, 13, 17, 19];

        assert_eq!(primes_below_n(10), primes_below_10);
        assert_eq!(primes_below_n(20), primes_below_20);
    }

    #[test]
    fn test_nth_prime_number() {
        let prime1 = 2;
        let prime5 = 11;
        let prime10 = 29;

        assert_eq!(nth_prime_number(1), prime1);
        assert_eq!(nth_prime_number(5), prime5);
        assert_eq!(nth_prime_number(10), prime10);
    }
}
