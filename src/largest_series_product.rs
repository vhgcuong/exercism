#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    SpanTooLong,
    InvalidDigit(char),
}

pub fn lsp(string_digits: &str, span: usize) -> Result<u64, Error> {
    // if span > string_digits.len() {
    //     return Err(Error::SpanTooLong);
    // }
    // let digits: Vec<u64> = string_digits
    //     .chars()
    //     .map(|c| {
    //         c.to_digit(10)
    //             .map(|d| d as u64)
    //             .ok_or(Error::InvalidDigit(c))
    //     })
    //     .collect::<Result<Vec<u64>, Error>>()?;
    // let largest_product = digits
    //     .windows(span)
    //     .map(|a| a.iter().product())
    //     .max()
    //     .unwrap();
    // Ok(largest_product)

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
