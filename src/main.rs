// use std::io;

// use crate::prime_factors::factors;
// use armstrong::is_armstrong_number;
// use nth_prime::nth_prime::nth;
// use raindrops::raindrops;
// use reverse::reverse;

// pub mod armstrong;
// pub mod nth_prime;
// pub mod prime_factors;
// pub mod sum_of_multiples;
// pub mod raindrops;
// pub mod reverse;

// pub mod luhn;
// use palindrome::*;
// pub mod palindrome;

pub mod perfect_numbers;
// use perfect_numbers::*;

fn main() {
    // let mut input = String::new();
    //
    // println!("Nhập một số nguyên: ");
    // io::stdin()
    //     .read_line(&mut input)
    //     .expect("Không thể đọc dữ liệu từ dòng lệnh");
    //
    // let input = match input.trim().parse::<u64>() {
    //     Ok(number) => number,
    //     Err(_) => {
    //         println!("Không thể chuyển đổi thành");
    //         return; // Thoát khỏi chương trình nếu không thể chuyển đổi
    //     }
    // };
    //
    // is_armstrong_number(input as u32);

   // nth(input as u32);

    // println!("{:?}", factors(input as u64));

    // println!("{}", sum_of_multiples::sum_of_multiples(4, &[3, 0]));

    // print!("{}", difference(input as u64));

    // println!("{}", raindrops(input as u32));

    // let digits: Vec<char> = vec!['1', '2', '3'];
    // println!("{:?}", digits[1..0]);

    // let code = "8569 6195 0383 3437";
    // println!("{}", luhn::is_valid(code));

    // println!("{:?}", largest_series_product::lsp("", 3));

    // Palindrome::new(123456);

    // palindrome_products(1, 99);

    println!("{:?}", perfect_numbers::classify(2));

}
