pub mod fibonacci;
pub mod triangle;
pub mod collatz;
pub mod pascal;
pub mod bigints;
pub mod words;
pub mod primes;



pub fn is_palindrome_number(n: u64) -> bool {
    let s = n.to_string();
    let rev = s.chars().rev().collect::<String>();

    s == rev
}

pub fn is_multiple(n: u64, d: u64) -> bool {
    n % d == 0
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


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_multiple() {
        assert!(!is_multiple(3, 2));
        assert!(is_multiple(4, 2));
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
}
