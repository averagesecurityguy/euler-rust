use num_bigint::BigUint;

// Raise x to the nth power
pub fn big_pow(n: u32, x:u32) -> String {
    let base = BigUint::from(n);

    return base.pow(x).to_str_radix(10);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_big_pow() {
        assert_eq!(big_pow(10, 0), String::from("1"));
        assert_eq!(big_pow(10, 1), String::from("10"));
        assert_eq!(big_pow(10, 2), String::from("100"));
        assert_eq!(big_pow(10, 3), String::from("1000"));
        assert_eq!(big_pow(7, 2), String::from("49"));
        assert_eq!(big_pow(2, 16), String::from("65536"));
    }
}
