// Define the Fibonacci struct and implement an iterator.
pub struct Fibonacci {
    curr: u64,
    next: u64,
}

impl Iterator for Fibonacci {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        if self.next > u64::MAX / 2 {
            None
        } else {
            let new_next = self.curr + self.next;
            let fib = self.curr;

            self.curr = self.next;
            self.next = new_next;

            Some(fib)
        }
    }
}

// Create a new Fibonacci struct
pub fn fibonacci() -> Fibonacci {
    Fibonacci { curr: 0, next: 1 }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fibonacci_sequence() {
        let mut f = fibonacci();

        assert_eq!(f.next().unwrap(), 0);
        assert_eq!(f.next().unwrap(), 1);
        assert_eq!(f.next().unwrap(), 1);
        assert_eq!(f.next().unwrap(), 2);
        assert_eq!(f.next().unwrap(), 3);
        assert_eq!(f.next().unwrap(), 5);
        assert_eq!(f.next().unwrap(), 8);
    }
}
