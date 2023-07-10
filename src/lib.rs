pub fn is_armstrong_number(num: u32) -> bool {
    if num < 10 {
        return true;
    }
    let num = u64::from(num);
    let digit_count = (num as f64).log10() as u32 + 1;
    let mut candidate = num as u64;
    let mut total = 0;
    while candidate > 0 {
        total += (candidate % 10).pow(digit_count);
        candidate /= 10;
    }
    num == total
}
