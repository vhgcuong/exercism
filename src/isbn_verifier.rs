pub fn is_valid_isbn(isbn: &str) -> bool {
    let nums = isbn.chars().filter(|&c| c.is_numeric() || c == 'X')
        .collect::<Vec<char>>();
    match nums.len() {
        10 => nums.into_iter().rev().enumerate().map(|(i, c)|
            match c {
                'X' => 10,
                _ => c.to_digit(10).unwrap() * (i as u32 + 1),
            }).sum::<u32>() % 11 == 0,
        _ => false
    }
}
