use time::{Duration, PrimitiveDateTime as DateTime};

// Returns a DateTime one billion seconds after start.
pub fn after(start: DateTime) -> DateTime {
    let gigasecond = 1e+9;

    let modified = start.checked_add(Duration::seconds(gigasecond as i64)).unwrap();

    modified
}
