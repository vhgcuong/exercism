#[derive(Debug, PartialEq, Eq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn sublist<T: PartialEq>(_first_list: &[T], _second_list: &[T]) -> Comparison {
    let fst_len = _first_list.len();
    let sec_len = _second_list.len();

    if fst_len > sec_len {
        if sec_len == 0 {
            return Comparison::Superlist;
        }
        for window in _first_list.windows(_second_list.len()) {
            if window.eq(_second_list) {
                return Comparison::Superlist;
            }
        }
    } else if fst_len == sec_len {
        if _first_list.eq(_second_list) {
            return Comparison::Equal;
        }
    } else if fst_len < sec_len {
        if fst_len == 0 {
            return Comparison::Sublist;
        }
        for window in _second_list.windows(_first_list.len()) {
            if window.eq(_first_list) {
                return Comparison::Sublist;
            }
        }
    }

    Comparison::Unequal
}
