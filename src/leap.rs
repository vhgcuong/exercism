pub fn is_leap_year(year: u64) -> bool {

    // match (year % 4, year % 100, year % 400) {
    //     (0, 1..=99, _) => true,
    //     (0, 0, 0) => true,
    //     (_, _, _) => false
    // }

    // if year % 4 == 0 {
    //     if year % 100 == 0 {
    //         return year % 400 == 0;
    //     }
    //     return true;
    // }
    //
    // return false

    if year % 100 == 0 {
        return year % 400 == 0;
    }

    year % 4 == 0
}
