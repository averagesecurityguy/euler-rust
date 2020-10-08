// Define the Triangle struct and implement an iterator.
pub struct Triangle {
    idx: u64,
    tri: u64,
}

impl Iterator for Triangle {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        if u64::MAX - self.tri < self.idx + 1 {
            None
        } else {
            self.idx += 1;
            self.tri += self.idx;

            Some(self.tri)
        }
    }
}

// Create a new Triange struct
pub fn triangle() -> Triangle {
    Triangle { idx: 0, tri: 0 }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_triangle_sequence() {
        let mut t = triangle();

        assert_eq!(t.next().unwrap(), 1);
        assert_eq!(t.next().unwrap(), 3);
        assert_eq!(t.next().unwrap(), 6);
        assert_eq!(t.next().unwrap(), 10);
        assert_eq!(t.next().unwrap(), 15);
        assert_eq!(t.next().unwrap(), 21);
        assert_eq!(t.next().unwrap(), 28);
    }
}
