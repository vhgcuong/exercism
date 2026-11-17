#[derive(Debug, PartialEq, Eq)]
pub enum Classification {
    Abundant,
    Perfect,
    Deficient,
}

pub fn classify(num: u64) -> Option<Classification> {
    // match num {
    //     0 => None,
    //     1 => Some(Classification::Deficient),
    //     _ => {
    //         let sum_aliquot: u64 = (1..=num/2).filter(|&x| num % x == 0).sum();
    //
    //         Some(match sum_aliquot.cmp(&num) {
    //             std::cmp::Ordering::Greater => Classification::Abundant,
    //             std::cmp::Ordering::Less => Classification::Deficient,
    //             _ => Classification::Perfect,
    //         })
    //     }
    // }

    match (1..=num / 2).filter(|i| num % i == 0).sum::<u64>() {
        _ if num == 0 => None,
        x if x > num => Some(Classification::Abundant),
        x if x < num => Some(Classification::Deficient),
        _ => Some(Classification::Perfect),
    }
}
