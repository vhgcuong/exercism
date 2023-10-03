///
///
/// # Arguments
///
/// * `array`:
/// * `key`:
///
/// returns: Option<usize>
///
/// # Examples
///
/// ```
///
/// ```

pub fn find(array: &[i32], key: i32) -> Option<usize> {
    if array.is_empty() {
        return None;
    }

    let mut low = 0;
    let mut hight = array.len() - 1;

    while low <= hight {
        let middle = (low + hight) / 2;
        if key == array[middle] {
            return Some(middle);
        }

        if key < array[middle] {
            if middle == 0 {
                return None;
            }
            hight = middle - 1;
        }

        if key > array[middle] {
            low = middle + 1;
        }
    }

    None
}
