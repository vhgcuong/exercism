pub fn verse(n: u32) -> String {
    if n < 1 {
        return String::from("No more bottles of beer on the wall, no more bottles of beer.\nGo to the store and buy some more, 99 bottles of beer on the wall.\n");
    }

    if n == 1 {
        return String::from(format!("{0} bottle of beer on the wall, {0} bottle of beer.\nTake it down and pass it around, no more bottles of beer on the wall.\n", n));
    }

    if n - 1 == 1 {
        return String::from(format!("{0} bottles of beer on the wall, {0} bottles of beer.\nTake one down and pass it around, {1} bottle of beer on the wall.\n", n, n-1))
    }

    return String::from(format!("{0} bottles of beer on the wall, {0} bottles of beer.\nTake one down and pass it around, {1} bottles of beer on the wall.\n", n, n-1))
}

pub fn sing(start: u32, end: u32) -> String {
    let mut text = String::new();
    let mut index = start;
    loop {
        text.push_str(&verse(index));

        if index > end {
            text.push_str("\n");
            index -= 1;
        } else {
            break;
        }
    }

    text
}
