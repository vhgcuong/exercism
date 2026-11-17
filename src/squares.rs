pub mod squares {
    pub fn square_of_sum(n: u64) -> u64 {
        let sum: u64 = (1..=n).sum();
        sum * sum
    }

    pub fn sum_of_squares(n: u64) -> u64 {
        n * (n+1) * (2*n+1) / 6
    }

    pub fn difference(n: u64) -> u64 {
        square_of_sum(n) - sum_of_squares(n)
    }
}
