use std::iter::once;

pub fn build_proverb(list: &[&str]) -> String {
    // let mut expected = String::new();
    //
    // if list.is_empty() {
    //     return expected;
    // }
    //
    // for i in 0..list.len() - 1 {
    //     if (i+1) < list.len() {
    //         expected.push_str(&format!("For want of a {} the {} was lost.\n", list[i], list[i+1]))
    //     }
    // }
    //
    // expected.push_str(&format!("And all for the want of a {}.", list[0]));
    //
    // return expected;

    match list.first() {
        None => String::new(),
        Some(word) => list.windows(2)
            .map(|w| format!("For want of a {} the {} was lost.\n", w[0], w[1]))
            .chain(once(format!("And all for the want of a {}.", word)))
            .collect(),
    }
}
