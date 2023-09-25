use std::{ collections::HashMap, sync::{Arc, Mutex}, thread };

pub fn frequency(input: &[&str], worker_count: usize) -> HashMap<char, usize> {
    let mut answers = HashMap::<char, usize>::new();
    if input.is_empty() {
        return answers;
    }

    for worker_num in 0..worker_count {

    }

    answers
}
