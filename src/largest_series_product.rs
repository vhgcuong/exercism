#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    SpanTooLong,
    InvalidDigit(char),
}

pub fn lsp(string_digits: &str, span: usize) -> Result<u64, Error> {
    match span {
        0 => Ok(1),
        _ => string_digits
            .chars()
            .map(|x| x.to_digit(10).ok_or(Error::InvalidDigit(x)).map(u64::from))
            .collect::<Result<Vec<u64>, Error>>()?
            .windows(span).map(|w| w.iter().product())
            .max()
            .ok_or(Error::SpanTooLong),
    }

}
