pub mod nth_prime {
    pub fn nth(n: u32) -> u32 {
        let mut nth_primes: Vec<u32> = Vec::new();
        let mut index = 0;

        loop {
            if !nth_primes.is_empty() && nth_primes.len() == (n + 1) as usize {
                return *nth_primes.get((n) as usize).unwrap();
            }

            if is_prime(index as f64) {
                nth_primes.push(index);
            }
            index += 1;
        }
    }

    pub fn is_prime(number: f64) -> bool {
        if number < 2.0 {
            return false;
        }

        let square_number = number.sqrt() as u32 + 1;
        for i in 2..square_number {
            if number as u32 % i == 0 {
                return false;
            }
        }

        true
    }
}

#[cfg(test)]
mod nth_prime_tests {
    use super::nth_prime::*;

    #[test]
    fn test_first_prime() {
        assert_eq!(nth(0), 2);
    }
    #[test]
    #[ignore]
    fn test_second_prime() {
        assert_eq!(nth(1), 3);
    }
    #[test]
    #[ignore]
    fn test_sixth_prime() {
        assert_eq!(nth(5), 13);
    }
    #[test]
    #[ignore]
    fn test_big_prime() {
        assert_eq!(nth(10_000), 104_743);
    }
}