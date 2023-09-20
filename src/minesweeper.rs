static NEIGHBOURHOOD_OFFSETS: &[(i32, i32)] = &[
    (-1, -1), (0, -1), (1, -1),
    (-1,  0),          (1,  0),
    (-1,  1), (0,  1), (1,  1),
];

pub fn annotate(minefield: &[&str]) -> Vec<String> {
    minefield.iter().enumerate().map(|(y, row)| {
        row.chars().enumerate().map(|(x, c)| match c {
            '*' => '*',
            _ => {
                let count = NEIGHBOURHOOD_OFFSETS.iter()
                    .filter(|&&(ox, oy)| {
                        let nx = x as i32 + ox;
                        let ny = y as i32 + oy;
                        nx >= 0 && nx < row.len() as i32 && ny >= 0 && ny < minefield.len() as i32
                    })
                    .filter(|&&(ox, oy)| {
                        let nx = x as i32 + ox;
                        let ny = y as i32 + oy;
                        minefield[ny as usize].chars().nth(nx as usize).unwrap() == '*'
                    })
                    .count();

                if count > 0 {
                    (count as u8 + b'0') as char
                } else {
                    ' '
                }
            }
        }).collect()
    }).collect()
}
