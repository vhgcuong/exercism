/// `Palindrome` is a newtype which only exists when the contained value is a palindrome number in base ten.
///
/// A struct with a single field which is used to constrain behavior like this is called a "newtype", and its use is
/// often referred to as the "newtype pattern". This is a fairly common pattern in Rust.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Palindrome(u64);

impl Palindrome {
    /// Create a `Palindrome` only if `value` is in fact a palindrome when represented in base ten. Otherwise, `None`.
    pub fn new(value: u64) -> Option<Palindrome> {
        if is_palindrome(&value) {
            return Some(Palindrome(value));
        }

        None
    }

    /// Get the value of this palindrome.
    pub fn into_inner(self) -> u64 {
        self.0
    }
}

pub fn palindrome_products(min: u64, max: u64) -> Option<(Palindrome, Palindrome)> {
    let mut result: Vec<u64> = vec![];

    for item in min..=max {
        for value in item..=max {
            let multiple: u64 = item * value;
            if is_palindrome(&multiple) && !result.contains(&multiple) {
                result.push(multiple);
            }
        }
    }

    if result.is_empty() || result.len() <= 1 {
        return None;
    }

    let min_palindrome = Palindrome::new(*result.iter().min().unwrap());
    let max_palindrome = Palindrome::new(*result.iter().max().unwrap());

    if min_palindrome.is_none() || max_palindrome.is_none() {
        return None;
    }

    Some((min_palindrome.unwrap(), max_palindrome.unwrap()))
}

fn is_palindrome(value: &u64) -> bool {
    let str_value: String = value.to_string().chars().rev().collect();
    str_value == value.to_string()
}
