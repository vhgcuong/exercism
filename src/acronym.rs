pub fn abbreviate(phrase: &str) -> String {
    let texts: Vec<&str> = phrase.split(|c: char| c.is_ascii_whitespace() || c == '_' || c == '-').collect();
    let mut result = String::from("");

    for text in texts {
        //  Lấy ký tự đầu tiên
        result.push_str(&text
            .chars()
            .take(1)
            .collect::<String>()
        );

        // Lấy ký tự viết hoa trong từ
        // HyperText -> T
        // GNU -> G
        result.push_str(&text
            .chars()
            .skip_while(|c| c.is_ascii_uppercase())
            .filter(|c| c.is_ascii_uppercase())
            .collect::<String>()
        );
    }

    result.to_uppercase()

    // phrase
    //     .split(|c: char| c.is_ascii_whitespace() || c == '_' || c == '-')
    //     .flat_map(|word| {
    //         word.chars().take(1).chain(
    //             word.chars()
    //                 .skip_while(|c| c.is_ascii_uppercase())
    //                 .filter(|c| c.is_ascii_uppercase()),
    //         )
    //     })
    //     .collect::<String>()
    //     .to_ascii_uppercase()
}
