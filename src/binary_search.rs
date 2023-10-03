use std::cmp::Ordering;

pub fn find<T: Ord>(array: &[T], key: T) -> Option<usize> {
    if array.is_empty() {
        return None;
    }

    let mut low = 0;
    let mut hight = array.len() - 1;

    while low <= hight {
        let mid = (low + hight) / 2;

        match array[mid].cmp(&key) {
            Ordering::Less => low = mid + 1,
            Ordering::Greater => {
                if mid == 0 {
                    return None;
                }
                hight = mid - 1
            },
            Ordering::Equal => return Some(mid),
        }
    }
    None
}
