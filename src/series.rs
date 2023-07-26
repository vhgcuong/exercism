pub fn series(digits: &str, len: usize) -> Vec<String> {
    // let mut series: Vec<String> = Vec::new();
    // let numbers: Vec<char> = digits.chars().collect();
    //
    // if numbers.len() < len {
    //     return series;
    // }
    //
    // for index in 0..=numbers.len() {
    //     if len == 0 {
    //         series.push("".to_string());
    //     } else {
    //         if (index + len) > numbers.len() {
    //             break;
    //         }
    //         series.push(numbers[index..(index + len)].iter().collect());
    //     }
    // }
    //
    // series

    if len == 0 {
        return vec!["".to_string(); digits.len() + 1];
    }
    digits
        .chars()
        .collect::<Vec<char>>()
        .windows(len)
        .map(|c| c.into_iter().collect::<String>())
        .collect()
}
