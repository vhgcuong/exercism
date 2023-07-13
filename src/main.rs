use std::io;
use exercism::squares::squares::{difference, square_of_sum};
// use crate::prime_factors::factors;
// use armstrong::is_armstrong_number;
// use nth_prime::nth_prime::nth;

// pub mod armstrong;
// pub mod nth_prime;
// pub mod prime_factors;

pub mod sum_of_multiples;

fn main() {
    let mut input = String::new();

    println!("Nhập một số nguyên: ");
    io::stdin()
        .read_line(&mut input)
        .expect("Không thể đọc dữ liệu từ dòng lệnh");

    let input = match input.trim().parse::<u64>() {
        Ok(number) => number,
        Err(_) => {
            println!("Không thể chuyển đổi thành");
            return; // Thoát khỏi chương trình nếu không thể chuyển đổi
        }
    };

    // is_armstrong_number(input as u32);

   // nth(input as u32);

    // println!("{:?}", factors(input as u64));

    // println!("{}", sum_of_multiples::sum_of_multiples(4, &[3, 0]));

    print!("{}", difference(input as u64));
}
