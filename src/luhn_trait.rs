pub trait Luhn {
    fn valid_luhn(&self) -> bool;
}

/// Here is the example of how to implement custom Luhn trait
/// for the &str type. Naturally, you can implement this trait
/// by hand for the every other type presented in the test suite,
/// but your solution will fail if a new type is presented.
/// Perhaps there exists a better solution for this problem?
impl<'a> Luhn for &'a str {
    fn valid_luhn(&self) -> bool {
        self.to_string().chars()
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

impl Luhn for String {
    fn valid_luhn(&self) -> bool {
        self.to_string().chars()
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

impl Luhn for u8 {
    fn valid_luhn(&self) -> bool {
        self.to_string().chars()
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

impl Luhn for u16 {
    fn valid_luhn(&self) -> bool {
        self.to_string().chars()
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

impl Luhn for u32 {
    fn valid_luhn(&self) -> bool {
        self.to_string().chars()
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


impl Luhn for u64 {
    fn valid_luhn(&self) -> bool {
        self.to_string().chars()
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

impl Luhn for usize {
    fn valid_luhn(&self) -> bool {
        self.to_string().chars()
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