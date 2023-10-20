/// Determines whether the supplied string is a valid ISBN number
pub fn is_valid_isbn(isbn: &str) -> bool {
    if isbn.is_empty() {
        return false;
    }

    let isbn_digit: Vec<_> = isbn.to_string().chars().filter(|c| c.is_ascii_digit()).collect();
    println!("{:?}", isbn_digit);

    true
    // todo!("Is {isbn:?} a valid ISBN number?");
}
