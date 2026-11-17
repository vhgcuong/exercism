use std::collections::HashSet;

pub fn find(sum: u32) -> HashSet<[u32; 3]> {
    let mut result: HashSet<[u32; 3]> = HashSet::new();
    for a in 1..=sum / 2 {
        for b in a..sum / 2 {
            if a + b > sum {
                break;
            }

            let c = sum - b - a;
            if c < a || c < b {
                continue;
            }

            if a * a + b * b == c * c {
                result.insert([a, b, c]);
            }
        }
    }

    result
}
