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

#[cfg(test)]
mod raindrops_tests {
    use super::*;

    #[test]
    fn test_two_pieces() {
        let input = vec!["nail", "shoe"];
        let expected = vec![
            "For want of a nail the shoe was lost.",
            "And all for the want of a nail.",
        ]
            .join("\n");
        assert_eq!(build_proverb(&input), expected);
    }
    // Notice the change in the last line at three pieces.
    #[test]
    #[ignore]
    fn test_three_pieces() {
        let input = vec!["nail", "shoe", "horse"];
        let expected = vec![
            "For want of a nail the shoe was lost.",
            "For want of a shoe the horse was lost.",
            "And all for the want of a nail.",
        ]
            .join("\n");
        assert_eq!(build_proverb(&input), expected);
    }
    #[test]
    #[ignore]
    fn test_one_piece() {
        let input = vec!["nail"];
        let expected = String::from("And all for the want of a nail.");
        assert_eq!(build_proverb(&input), expected);
    }
    #[test]
    #[ignore]
    fn test_zero_pieces() {
        let input: Vec<&str> = vec![];
        let expected = String::new();
        assert_eq!(build_proverb(&input), expected);
    }
    #[test]
    #[ignore]
    fn test_full() {
        let input = vec![
            "nail", "shoe", "horse", "rider", "message", "battle", "kingdom",
        ];
        let expected = vec![
            "For want of a nail the shoe was lost.",
            "For want of a shoe the horse was lost.",
            "For want of a horse the rider was lost.",
            "For want of a rider the message was lost.",
            "For want of a message the battle was lost.",
            "For want of a battle the kingdom was lost.",
            "And all for the want of a nail.",
        ]
            .join("\n");
        assert_eq!(build_proverb(&input), expected);
    }
    #[test]
    #[ignore]
    fn test_three_pieces_modernized() {
        let input = vec!["pin", "gun", "soldier", "battle"];
        let expected = vec![
            "For want of a pin the gun was lost.",
            "For want of a gun the soldier was lost.",
            "For want of a soldier the battle was lost.",
            "And all for the want of a pin.",
        ]
            .join("\n");
        assert_eq!(build_proverb(&input), expected);
    }
}