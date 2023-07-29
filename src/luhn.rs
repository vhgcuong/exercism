/// Check a Luhn checksum.
pub fn is_valid(code: &str) -> bool {
    let mut sum = 0;
    let numbers: Vec<char> = code.chars().rev().collect();
    let mut index = 0;

    for number in numbers {
        if number.is_whitespace() {
            continue;
        }

        match number.to_digit(10) {
            Some(value) => {
                if index % 2 == 0 {
                    sum += value;
                } else {
                    if value > 4 {
                        sum += value * 2 - 9;
                    } else {
                        sum += value * 2;
                    }
                }

                index += 1;
            },
            None => return false
        }
    }

    index > 1 && sum % 10 == 0
}
