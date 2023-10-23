/// Determines whether the supplied string is a valid ISBN number
pub fn is_valid_isbn(isbn: &str) -> bool {
    if isbn.is_empty() {
        return false;
    }

    let mut isbn_digit: Vec<_> = isbn.to_string().chars().filter(|&c| c != '-').collect();
    let last_isbn = isbn_digit.pop().unwrap();

    if isbn_digit.iter().any(|c| c.is_ascii_alphabetic()) {
        return false;
    }

    isbn_digit
        .iter()
        .rev()
        .filter_map(|ch| ch.to_digit(10))
        .enumerate()
        .for_each(|(key, value)| {

        });

    true
    // todo!("Is {isbn:?} a valid ISBN number?");
}
