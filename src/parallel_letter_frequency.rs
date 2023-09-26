use std::{ collections::HashMap, thread, sync::Arc };

pub fn frequency(input: &[&str], worker_count: usize) -> HashMap<char, usize> {
    let data: Vec<String> = input.iter().map(|&s| s.to_ascii_lowercase()).collect();
    let arc = Arc::new(data);

    let mut handlers: Vec<_> = Vec::with_capacity(worker_count);
    (0..worker_count).for_each(|worker| {
        let arc_clone = Arc::clone(&arc);
        let handle = thread::spawn(move || {
            let mut counts = HashMap::<char, usize>::new();

            arc_clone.iter().enumerate().for_each(|(index, value)| {
                if index % worker_count == worker {
                    value.chars().filter(|ch| ch.is_alphabetic()).for_each(|ch| *counts.entry(ch).or_default() += 1)
                }
            });

            counts
        });
        handlers.push(handle);
    });

    let mut counters = HashMap::<char, usize>::new();
    for handler in handlers {
        for (key, value) in handler.join().unwrap() {
            let _ = *counters
                .entry(key)
                .and_modify(|count| *count += value)
                .or_insert(value);
        }
    }

    counters
}
