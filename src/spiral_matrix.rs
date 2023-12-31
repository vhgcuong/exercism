pub fn spiral_matrix(size: u32) -> Vec<Vec<u32>> {
    let mut matrix = vec![vec![0; size as usize]; size as usize];
    let mut dir: i8 = 1;
    let mut pos: (i8, i8) = (0, -1);
    let mut count = 1;
    let mut length = size;
    while count <= size * size {
        (0..length).for_each(|_| {
            pos = (pos.0, pos.1 + dir);
            matrix[pos.0 as usize][pos.1 as usize] = count;
            count += 1;
        });

        length -= 1;
        (0..length).for_each(|_| {
            pos = (pos.0 + dir, pos.1);
            matrix[pos.0 as usize][pos.1 as usize] = count;
            count += 1;
        });

        dir *= -1;
    }

    matrix
}
