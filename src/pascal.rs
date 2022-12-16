// Define the PascalRow struct and implement an iterator.
pub struct PascalRow {
    // curr: u64,
    next: u64,
    n: u64,
    k: u64,
}

impl Iterator for PascalRow {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        if self.next > u64::MAX / 2 {
            None
        } else if self.next == 0 {
            None
        } else {
            let pas = self.next;
            let new_next: u64 = ((self.next * (self.n + 1 - self.k)) / self.k).into();

            self.next = new_next;
            self.k = self.k + 1;

            Some(pas)
        }
    }
}

// Create a new PascalRow struct
pub fn pascal_row(n: u64) -> PascalRow {
    PascalRow { next: 1, n: n, k: 1}
}

pub fn pascal_kth(n: u64, k: u64) -> Option<u64> {
    if k > n {
        return None;
    }

    let pr = pascal_row(n);
    let pos = k as usize;
    let mut kth: u64 = 1;

    for (_, val) in pr.enumerate().skip(pos).take(1) { kth = val; }

    Some(kth)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pascal_row_sequence() {
        let mut pr = pascal_row(5);

        assert_eq!(pr.next().unwrap(), 1);
        assert_eq!(pr.next().unwrap(), 5);
        assert_eq!(pr.next().unwrap(), 10);
        assert_eq!(pr.next().unwrap(), 10);
        assert_eq!(pr.next().unwrap(), 5);
        assert_eq!(pr.next().unwrap(), 1);
        assert_eq!(pr.next().is_none(), true);
    }

    #[test]
    fn test_pascal_kth() {
        assert_eq!(pascal_kth(0,0), Some(1));
        assert_eq!(pascal_kth(1,0), Some(1));
        assert_eq!(pascal_kth(1,1), Some(1));
        assert_eq!(pascal_kth(4,2), Some(6));
        assert_eq!(pascal_kth(5,3), Some(10));
        assert_eq!(pascal_kth(6,3), Some(20));
    }
}
