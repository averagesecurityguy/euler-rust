// Define the Triangle struct and implement an iterator.
pub struct Collatz {
    c: u64,
}

impl Iterator for Collatz {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        if self.c == 1 {
            None
        } else if self.c % 2 == 1 && self.c > (u64::MAX - 1) / 3 {
            None
        } else if self.c % 2 == 1 {
            self.c = (3 * self.c) + 1;
            Some(self.c)
        } else {
            self.c = self.c / 2;
            Some(self.c)
        }
    }
}

// Create a new Collatz struct
pub fn collatz(n: u64) -> Collatz {
    Collatz { c: n }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_collatz_sequence() {
        let mut c = collatz(13);

        assert_eq!(c.next(), Some(40));
        assert_eq!(c.next(), Some(20));
        assert_eq!(c.next(), Some(10));
        assert_eq!(c.next(), Some(5));
        assert_eq!(c.next(), Some(16));
        assert_eq!(c.next(), Some(8));
        assert_eq!(c.next(), Some(4));
        assert_eq!(c.next(), Some(2));
        assert_eq!(c.next(), Some(1));
        assert_eq!(c.next(), None);
    }
}
