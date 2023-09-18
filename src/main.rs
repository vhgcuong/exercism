use std::thread;
use std::sync::mpsc;

use exercism::alphametics;
fn main() {
    let input = "SEND + MORE == MONEY";

    // Số lượng luồng bạn muốn sử dụng
    let num_threads = 4;

    let (tx, rx) = mpsc::channel();

    for _ in 0..num_threads {
        let tx = tx.clone();
        let input = input.to_string();

        thread::spawn(move || {
            let solution = alphametics::solve(&input);
            tx.send(solution).unwrap();
        });
    }

    let mut solutions = Vec::new();
    for _ in 0..num_threads {
        let solution = rx.recv().unwrap();
        if let Some(result) = solution {
            solutions.push(result);
        }
    }

    // Xử lý các giải pháp từ các luồng con ở đây
    println!("{:?}", solutions);
}
