pub mod alphametics;

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

    alphametics::solve("HE + SEES + THE == LIGHT");
}
