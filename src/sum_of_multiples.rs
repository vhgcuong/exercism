pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
    // let mut numbers: Vec<u32> = vec![];
    //
    // for item in factors {
    //     if *item == 0 {
    //         continue;
    //     }
    //
    //     for i in 0..limit {
    //         if i % item == 0 && !numbers.contains(&i){
    //             numbers.push(i);
    //         }
    //     }
    // }
    //
    // numbers.iter().sum()

    (1..limit)
        .filter(|i| factors.iter().any(|&f| f != 0 && i % f == 0))
        .sum()
}
