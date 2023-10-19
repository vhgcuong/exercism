/// Return the Hamming distance between the strings,
/// or None if the lengths are mismatched.
pub fn hamming_distance(s1: &str, s2: &str) -> Option<usize> {
    if s1.len() != s2.len() {
        return None;
    }

    if s1.is_empty() || s2 == s1 {
        return Some(0);
    }

    let mut count = 0;
    for (key, ch) in s1.chars().enumerate() {
        if s2.as_bytes()[key] != ch as u8 {
            count += 1;
        }
    }
    Some(count)
}
