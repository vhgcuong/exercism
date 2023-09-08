pub fn abbreviate(phrase: &str) -> String {
    let texts: Vec<String> = phrase.split_whitespace().map(|x| x.to_string()).collect();
    let mut result = String::from("");

    for text in texts {
        let text = text.trim_matches(|c| c == '_' || c == ':');
        let characters: Vec<_> = text.chars().collect();
        if characters[0].is_lowercase() {
            result.push_str(&characters[0].to_string());
            match text.find('-') {
                Some(index) => result.push_str(&characters[index+1].to_string()),
                None => (),
            };
        } else {
            let char_uppercase: String = characters.iter().filter(|&x| x.is_uppercase()).collect();
            if char_uppercase == text {
                result.push_str(&characters[0].to_string());
            } else {
                result.push_str(char_uppercase.as_str());
            }
        }
    }

    result.to_uppercase()
}
