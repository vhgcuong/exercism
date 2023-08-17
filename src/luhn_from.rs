pub struct Luhn {
    code: String
}

impl Luhn {
    pub fn is_valid(&self) -> bool {
        self.code.chars()
            .rev()
            .filter(|c| !c.is_whitespace())
            .try_fold((0, 0), |(sum, count), val| {
                val.to_digit(10)
                    .map(|num| if count % 2 == 1 { num * 2 } else { num })
                    .map(|num| if num > 9 { num - 9 } else { num })
                    .map(|num| (num + sum, count + 1))
            }).map_or(false, |(sum, count)| sum % 10 == 0 && count > 1)
    }
}

/// Here is the example of how the From trait could be implemented
/// for the &str type. Naturally, you can implement this trait
/// by hand for the every other type presented in the test suite,
/// but your solution will fail if a new type is presented.
/// Perhaps there exists a better solution for this problem?
impl<T> From<T> for Luhn where T: ToString {
    fn from(input: T) -> Self {
        Luhn {
            code: input.to_string()
        }
    }
}
