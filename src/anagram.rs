use std::collections::HashSet;

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    let word_lowercase = word.to_lowercase();
    let mut word_sort: Vec<_> = word_lowercase.chars().collect();
    word_sort.sort_unstable();

    let mut answers = HashSet::new();
    for &possible_anagram in possible_anagrams {
        let possible_anagram_lowercase = possible_anagram.to_lowercase();

        let mut p_anagram_sort: Vec<_> = possible_anagram_lowercase.chars().collect();
        p_anagram_sort.sort_unstable();

        if word_sort == p_anagram_sort && word_lowercase != possible_anagram_lowercase {
            answers.insert(possible_anagram);
        }
    }

    answers
}
