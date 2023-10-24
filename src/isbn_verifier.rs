/// Determines whether the supplied string is a valid ISBN number
pub fn is_valid_isbn(isbn: &str) -> bool {
    if isbn.is_empty() {
        return false;
    }

    let isbn_digit: Vec<char> = isbn.to_string().chars().filter(|&c| c != '-').collect();
    if isbn_digit.len() != 10 {
        return false;
    }

    let mut sum_isbn: u32 = 0;
    for (key, value) in isbn_digit.iter().rev().enumerate() {
        if key == 0 {
            match value {
                'X' => sum_isbn += 10,
                '0'..='9' => sum_isbn += value.to_digit(10).unwrap(),
                _ => return false
            }
        } else {
            if !value.is_digit(10) {
                return false;
            }
            sum_isbn += (key as u32 + 1) * value.to_digit(10).unwrap();
        }
    };

    sum_isbn % 11 == 0
}
