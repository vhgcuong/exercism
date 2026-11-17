use std::{ collections::HashMap, thread, sync::Arc };
use std::sync::Mutex;

pub fn frequency(input: &[&str], worker_count: usize) -> HashMap<char, usize> {
    let counters = Arc::new(Mutex::new(HashMap::<char, usize>::new()));
    let data: Vec<String> = input.iter().map(|&s| s.to_ascii_lowercase()).collect();

    thread::scope(|scope| {
        let chunk_size = data.len() / worker_count + 1;
        for item in data.chunks(chunk_size) {
            let mutex = counters.clone();
            scope.spawn(move || {
                for str in item {
                    str.chars().filter(|ch| ch.is_alphabetic()).for_each(|ch| {
                        *mutex.lock().unwrap().entry(ch).or_default() += 1;
                    })
                }
            });
        }
    });

    let m = counters.lock().unwrap();
    m.to_owned()
}
