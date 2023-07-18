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

    /// Cach 2
    // input.chars().rev().collect()

    /// Cach 3
    input.rsplit("").collect()
}

#[cfg(test)]
mod reverse_tests {
    use super::*;

    fn process_reverse_case(input: &str, expected: &str) {
        assert_eq!(&reverse(input), expected)
    }

    #[test]
    /// empty string
    fn test_an_empty_string() {
        process_reverse_case("", "");
    }

    #[test]
    #[ignore]
    /// a word
    fn test_a_word() {
        process_reverse_case("robot", "tobor");
    }

    #[test]
    #[ignore]
    /// a capitalized word
    fn test_a_capitalized_word() {
        process_reverse_case("Ramen", "nemaR");
    }

    #[test]
    #[ignore]
    /// a sentence with punctuation
    fn test_a_sentence_with_punctuation() {
        process_reverse_case("I'm hungry!", "!yrgnuh m'I");
    }

    #[test]
    #[ignore]
    /// a palindrome
    fn test_a_palindrome() {
        process_reverse_case("racecar", "racecar");
    }

    #[test]
    #[ignore]
    /// an even-sized word
    fn test_an_even_sized_word() {
        process_reverse_case("drawer", "reward");
    }

    #[test]
    #[ignore]
    /// wide characters
    fn test_wide_characters() {
        process_reverse_case("子猫", "猫子");
    }

    #[test]
    #[ignore]
    #[cfg(feature = "grapheme")]
    /// grapheme clusters
    fn test_grapheme_clusters() {
        process_reverse_case("uüu", "uüu");
    }
}
