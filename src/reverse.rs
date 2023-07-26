pub fn reverse(input: &str) -> String {
    // let mut str_reverse = String::new();
    // if input.is_empty() {
    //     return str_reverse;
    // }
    //
    // for item in input.chars().rev() {
    //     str_reverse.push(item);
    // }
    //
    // str_reverse

    // Cach 2
    // input.chars().rev().collect()

    // Cach 3
    input.rsplit("").collect()
}
