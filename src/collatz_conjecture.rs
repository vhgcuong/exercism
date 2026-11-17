pub fn collatz(n: u64) -> Option<u64> {
    let mut number: u64 = n;
    let mut index: u64 = 0;

    if number == 0 {
        return None;
    }

    while number != 1 {
        match number % 2 {
            0 => number /= 2,
            _ => number = number.checked_mul(3)?.checked_add(1)?
        }
        index += 1;
    };

    Some(index)
}
