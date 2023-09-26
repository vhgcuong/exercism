use std::{ collections::HashMap, thread };
use std::cmp::min;

pub fn frequency(input: &[&str], worker_count: usize) -> HashMap<char, usize> {
    let input = input.join("");
    if input.len() == 0 {
        return HashMap::new();
    }

    let counter = |input: String| {
        let mut answer : HashMap<char, usize> = HashMap::new();
        input.chars().for_each(|c| {
            if c.is_alphabetic() {
                *answer.entry(c.to_ascii_lowercase()).or_default() += 1;
            }
        });
        answer
    };

    let real_worker_count = min(input.len(), worker_count);
    let mut work_length = (input.len() / real_worker_count).max(1);
    if work_length * real_worker_count < input.len() {
        work_length = work_length + 1;
    }

    let mut handlers: Vec<_> = Vec::with_capacity(real_worker_count);
    let mut churn = input.chars();

    for _ in 0..real_worker_count {
        let chunk = churn.by_ref().take(work_length).collect::<String>();
        handlers.push(thread::spawn(move || {
            counter(chunk)
        }));
    }

    let mut counts: HashMap<char, usize> = HashMap::new();
    for handler in handlers {
        for (key, val) in handler.join().unwrap() {
            *counts.entry(key).or_default() += val;
        }
    }

    counts
}
