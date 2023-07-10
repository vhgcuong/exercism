use std::io;
use exercism::is_armstrong_number;

fn main() {
    let mut input = String::new();

    println!("Nhập một số nguyên: ");
    io::stdin()
        .read_line(&mut input)
        .expect("Không thể đọc dữ liệu từ dòng lệnh");

    let input = match input.trim().parse::<i32>() {
        Ok(number) => {
            number
        }
        Err(_) => {
            println!("Không thể chuyển đổi thành i32");
            return; // Thoát khỏi chương trình nếu không thể chuyển đổi
        }
    };

    is_armstrong_number(input as u32);
}
