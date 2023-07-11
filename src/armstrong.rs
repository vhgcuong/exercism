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

#[cfg(test)]
mod armstrong_tests {
    use super::*;

    #[test]
    fn test_zero_is_an_armstrong_number() {
        assert!(is_armstrong_number(0))
    }
    #[test]
    #[ignore]
    fn test_single_digit_numbers_are_armstrong_numbers() {
        assert!(is_armstrong_number(5))
    }
    #[test]
    #[ignore]
    fn test_there_are_no_2_digit_armstrong_numbers() {
        assert!(!is_armstrong_number(10))
    }
    #[test]
    #[ignore]
    fn test_three_digit_armstrong_number() {
        assert!(is_armstrong_number(153))
    }
    #[test]
    #[ignore]
    fn test_three_digit_non_armstrong_number() {
        assert!(!is_armstrong_number(100))
    }
    #[test]
    #[ignore]
    fn test_four_digit_armstrong_number() {
        assert!(is_armstrong_number(9474))
    }
    #[test]
    #[ignore]
    fn test_four_digit_non_armstrong_number() {
        assert!(!is_armstrong_number(9475))
    }
    #[test]
    #[ignore]
    fn test_seven_digit_armstrong_number() {
        assert!(is_armstrong_number(9_926_315))
    }
    #[test]
    #[ignore]
    fn test_seven_digit_non_armstrong_number() {
        assert!(!is_armstrong_number(9_926_316))
    }
    #[test]
    #[ignore]
    fn test_nine_digit_armstrong_number() {
        assert!(is_armstrong_number(912_985_153));
    }
    #[test]
    #[ignore]
    fn test_nine_digit_non_armstrong_number() {
        assert!(!is_armstrong_number(999_999_999));
    }
    #[test]
    #[ignore]
    fn test_ten_digit_non_armstrong_number() {
        assert!(!is_armstrong_number(3_999_999_999));
    }
    // The following number has an Armstrong sum equal to 2^32 plus itself,
// and therefore will be detected as an Armstrong number if you are
// incorrectly using wrapping arithmetic.
    #[test]
    #[ignore]
    fn test_properly_handles_overflow() {
        assert!(!is_armstrong_number(4_106_098_957));
    }
}
