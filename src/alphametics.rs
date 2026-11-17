use std::iter::FromIterator;
use std::collections::{HashMap, HashSet};
use itertools::Itertools;

pub fn solve(input: &str) -> Option<HashMap<char, u8>> {
    let mut iter = input.split("==");

    let sum: Vec<&str> = iter.next().unwrap().split("+").map(|x| x.trim()).collect();
    let answer: &str = iter.next().unwrap().trim();

    // sum cannot be shorter than addends
    if sum.iter().any(|x| x.len() > answer.len()) {
        return None;
    }

    let alphabet: HashSet<char> = HashSet::from_iter(input.chars().filter(char::is_ascii_alphabetic));
    let perms = (0..=9).permutations(alphabet.len());

    for perm in perms {
        let value_map: HashMap<char, u8> = HashMap::from_iter(alphabet.iter().cloned().zip(perm.iter().cloned()));

        // no leading zeroes
        if value_map[&answer.chars().nth(0).unwrap()] == 0 {
            continue;
        }

        let answer_value = answer.chars().rev().enumerate()
            .map(|(i, c)| (value_map[&c] as u64) * 10_u64.pow(i as u32))
            .sum::<u64>();

        let mut sum_value: u64 = 0;
        for sum_component in &sum {
            // no leading zeroes
            if value_map[&sum_component.chars().nth(0).unwrap()] == 0 {
                continue;
            }
            sum_value += sum_component.chars().rev().enumerate()
                .map(|(i, c)| (value_map[&c] as u64) * 10_u64.pow(i as u32))
                .sum::<u64>();
        }

        if sum_value == answer_value {
            return Some(value_map);
        }
    }

    None
}
