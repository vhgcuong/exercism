pub fn annotate(minefield: &[&str]) -> Vec<String> {
    if minefield.is_empty() {
        return Vec::new();
    }

    let cleaned_strs: Vec<Vec<char>> = minefield
        .iter()
        .map(|x| x.chars().collect())
        .collect();

    let (x_len, y_len) = (cleaned_strs.len(), cleaned_strs[0].len());

    let mut result = Vec::new();

    for (x, row) in cleaned_strs.iter().enumerate() {
        let mut result_row = String::new();
        for (y, item) in row.iter().enumerate() {
            if *item == '*' {
                result_row.push('*');
            } else {
                let count = count_mines(&cleaned_strs, x as isize, y as isize, x_len as isize, y_len as isize);
                if count > 0 {
                    result_row.push(char::from_digit(count as u32, 10).unwrap());
                } else {
                    result_row.push(' ');
                }
            }
        }
        result.push(result_row);
    }

    result
}

fn count_mines(cleaned_strs: &Vec<Vec<char>>, x: isize, y: isize, x_len: isize, y_len: isize) -> u8 {
    let mut count: u8 = 0;

    for dx in -1..=1 {
        for dy in -1..=1 {
            if dx == 0 && dy == 0 {
                continue;
            }
            let new_x = x + dx;
            let new_y = y + dy;

            if new_x >= 0 && new_x < x_len && new_y >= 0 && new_y < y_len {
                if cleaned_strs[new_x as usize][new_y as usize] == '*' {
                    count += 1;
                }
            }
        }
    }

    count
}
