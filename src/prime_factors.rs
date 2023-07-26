pub fn factors(n: u64) -> Vec<u64> {
    let mut number = n;
    let mut prime_factors = vec![];
    while number > 1 {
        let i = (2..number+1).find(|x| number % x == 0 ).unwrap();
        prime_factors.push(i);
        number /= i;
    }
    prime_factors
}
