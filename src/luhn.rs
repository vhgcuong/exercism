/// Check a Luhn checksum.
pub fn is_valid(code: &str) -> bool {
    let numbers: Vec<_> = code.replace(" ", "").chars().rev().collect();
    if numbers.len() <= 1 {
        return false;
    }

    let is_numeric = numbers.iter().all(|x| x.is_numeric());
    if is_numeric {
        let mut sum: u32 = 0;
        for (index, value) in numbers.iter().enumerate() {
            if let Some(number) = value.to_digit(10) {
                match index % 2 == 0 {
                    true => sum += number,
                    false => {
                        sum += match number * 2 >= 10 {
                            true => number * 2 - 9,
                            false => number * 2
                        }
                    }
                }
            }
        }

        return sum % 10 == 0;
    }

    false
}
