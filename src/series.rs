pub fn series(digits: &str, len: usize) -> Vec<String> {
    let mut series: Vec<String> = Vec::new();
    let numbers: Vec<char> = digits.chars().collect();

    if numbers.len() < len {
        return series;
    }

    for index in 0..=numbers.len() {
        if len == 0 {
            series.push("".to_string());
        } else {
            if (index + len) > numbers.len() {
                break;
            }
            series.push(numbers[index..(index + len)].iter().collect());
        }
    }

    series
}

#[cfg(test)]
mod series_tests {
    use super::*;

    #[test]
    fn test_with_zero_length() {
        let expected = vec!["".to_string(); 6];
        assert_eq!(series("92017", 0), expected);
    }

    #[test]
    #[ignore]
    fn test_with_length_2() {
        let expected = vec![
            "92".to_string(),
            "20".to_string(),
            "01".to_string(),
            "17".to_string(),
        ];
        assert_eq!(series("92017", 2), expected);
    }

    #[test]
    #[ignore]
    fn test_with_numbers_length() {
        let expected = vec!["92017".to_string()];
        assert_eq!(series("92017", 5), expected);
    }

    #[test]
    #[ignore]
    fn test_too_long() {
        let expected: Vec<String> = vec![];
        assert_eq!(series("92017", 6), expected);
    }

    #[test]
    #[ignore]
    fn test_way_too_long() {
        let expected: Vec<String> = vec![];
        assert_eq!(series("92017", 42), expected);
    }
}
