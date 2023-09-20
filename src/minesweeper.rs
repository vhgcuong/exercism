pub fn annotate(minefield: &[&str]) -> Vec<String> {
    let mut result = Vec::new();

    for (x, row) in minefield.iter().enumerate() {
        let mut result_row = String::new();
        for (y, item) in row.chars().enumerate() {
            if item == '*' {
                result_row.push('*');
            } else {
                let mut count = 0;
                for i in x.saturating_sub(1)..=(x + 1) {
                    for j in y.saturating_sub(1)..=(y + 1) {
                        if i < minefield.len() && j < row.len() && minefield[i].chars().nth(j) == Some('*') {
                            count += 1;
                        }
                    }
                }
                if count > 0 {
                    result_row.push_str(&count.to_string());
                } else {
                    result_row.push(' ');
                }
            }
        }
        result.push(result_row);
    }

    result
}
