pub fn nth(n: u32) -> u32 {
    let mut nth_primes: Vec<u32> = Vec::new();
    let mut index = 0;

    loop {
        if nth_primes.last() == n {
            return *nth_primes.get(n - 1);
        }

        if (is_prime(index)) {
            nth_primes.push(index);
        }


    }

    nth_primes.last().unwrap();
}

fn is_prime(number: f64) -> bool {
    if number < 2 {
        return false;
    }

    let square_number = number.sqrt() as u32 + 1;
    for i in 2..square_number {
        if number % i == 0 {
            return false;
        }
    }

    true
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_first_prime() {
        assert_eq!(np::nth(0), 2);
    }
    #[test]
    #[ignore]
    fn test_second_prime() {
        assert_eq!(np::nth(1), 3);
    }
    #[test]
    #[ignore]
    fn test_sixth_prime() {
        assert_eq!(np::nth(5), 13);
    }
    #[test]
    #[ignore]
    fn test_big_prime() {
        assert_eq!(np::nth(10_000), 104_743);
    }
}