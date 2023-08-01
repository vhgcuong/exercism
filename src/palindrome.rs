/// `Palindrome` is a newtype which only exists when the contained value is a palindrome number in base ten.
///
/// A struct with a single field which is used to constrain behavior like this is called a "newtype", and its use is
/// often referred to as the "newtype pattern". This is a fairly common pattern in Rust.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Palindrome(u64);

impl Palindrome {
    /// Create a `Palindrome` only if `value` is in fact a palindrome when represented in base ten. Otherwise, `None`.
    pub fn new(value: u64) -> Option<Palindrome> {
        let str_value: String = value.to_string().chars().rev().collect();
        match str_value == value.to_string() {
            true => Some(Palindrome(value)),
            false => None
        }
    }

    /// Get the value of this palindrome.
    pub fn into_inner(self) -> u64 {
        self.0
    }
}

pub fn palindrome_products(min: u64, max: u64) -> Option<(Palindrome, Palindrome)> {
    let mut products: Vec<u64> = Vec::new();
    for item in min..=max {
        for value in min..=max {
            let multiple: u64 = item * value;
            if !products.contains(&multiple) && Palindrome::new(multiple) != None {
                products.push(multiple);
            }

        }
    }


    Some((
        Palindrome(products.iter().min().unwrap_or_else(None)),
        Palindrome(1)
    ))
}
