pub fn annotate(minefield: &[&str]) -> Vec<String> {
    if minefield.len() == 0 {
        return vec![];
    }

    let mut cleaned_strs = minefield
        .iter()
        .map(|x| x.chars().map(|ch| ch.to_string()).collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let (x_len, y_len) = {
        ( cleaned_strs.len(), cleaned_strs.get(0).map(|x| x.len()).unwrap())
    };

    let updated_strs = cleaned_strs.clone(); // Clone the original vector
    for (x, row) in updated_strs.iter().enumerate() {
        for (y, item) in row.iter().enumerate() {
            let mut count: u8 = 0;
            match item.as_str() {
                "*" => { },
                _ => {
                    match (x > 0, y > 0, x+1 < x_len, y+1 < y_len) {
                        (true, false, false, false) => {
                            if updated_strs[x-1][y] == "*" {
                                count += 1;
                            }
                        },
                        (false, true, false, false) => {
                            if updated_strs[x][y-1] == "*" {
                                count += 1;
                            }
                        },
                        (false, false, true, false) => {
                            if updated_strs[x+1][y] == "*" {
                                count += 1;
                            }
                        },
                        (false, false, false, true) => {
                            if updated_strs[x][y+1] == "*" {
                                count += 1;
                            }
                        },
                        (true, true, false, false) => {
                            if updated_strs[x-1][y] == "*" {
                                count += 1;
                            }
                            if updated_strs[x][y-1] == "*" {
                                count += 1;
                            }

                            if updated_strs[x-1][y-1] == "*" {
                                count += 1;
                            }
                        },
                        (true, false, true, false) => {
                            if updated_strs[x-1][y] == "*" {
                                count += 1;
                            }
                            if updated_strs[x+1][y] == "*" {
                                count += 1;
                            }
                        },
                        (true, false, false, true) => {
                            if updated_strs[x-1][y] == "*" {
                                count += 1;
                            }
                            if updated_strs[x][y+1] == "*" {
                                count += 1;
                            }

                            if updated_strs[x-1][y+1] == "*" {
                                count += 1;
                            }
                        },
                        (false, true, false, true) => {
                            if updated_strs[x][y-1] == "*" {
                                count += 1;
                            }
                            if updated_strs[x][y+1] == "*" {
                                count += 1;
                            }
                        },
                        (false, true, true, false) => {
                            if updated_strs[x][y-1] == "*" {
                                count += 1;
                            }
                            if updated_strs[x+1][y] == "*" {
                                count += 1;
                            }

                            if updated_strs[x+1][y-1] == "*" {
                                count += 1;
                            }
                        },
                        (false, false, true, true) => {
                            if updated_strs[x][y+1] == "*" {
                                count += 1;
                            }
                            if updated_strs[x+1][y] == "*" {
                                count += 1;
                            }

                            if updated_strs[x+1][y+1] == "*" {
                                count += 1;
                            }
                        },
                        (true, true, true, false) => {
                            if updated_strs[x-1][y] == "*" {
                                count += 1;
                            }
                            if updated_strs[x][y-1] == "*" {
                                count += 1;
                            }
                            if updated_strs[x+1][y] == "*" {
                                count += 1;
                            }

                            if updated_strs[x-1][y-1] == "*" {
                                count += 1;
                            }
                            if updated_strs[x+1][y-1] == "*" {
                                count += 1;
                            }
                        },
                        (true, true, false, true) => {
                            if updated_strs[x-1][y] == "*" {
                                count += 1;
                            }
                            if updated_strs[x][y-1] == "*" {
                                count += 1;
                            }
                            if updated_strs[x][y+1] == "*" {
                                count += 1;
                            }

                            if updated_strs[x-1][y-1] == "*" {
                                count += 1;
                            }
                            if updated_strs[x-1][y+1] == "*" {
                                count += 1;
                            }
                        },
                        (true, false, true, true) => {
                            if updated_strs[x-1][y] == "*" {
                                count += 1;
                            }
                            if updated_strs[x+1][y] == "*" {
                                count += 1;
                            }
                            if updated_strs[x][y+1] == "*" {
                                count += 1;
                            }

                            if updated_strs[x+1][y+1] == "*" {
                                count += 1;
                            }
                            if updated_strs[x-1][y+1] == "*" {
                                count += 1;
                            }
                        },
                        (false, true, true, true) => {
                            if updated_strs[x][y-1] == "*" {
                                count += 1;
                            }
                            if updated_strs[x+1][y] == "*" {
                                count += 1;
                            }
                            if updated_strs[x][y+1] == "*" {
                                count += 1;
                            }

                            if updated_strs[x+1][y+1] == "*" {
                                count += 1;
                            }
                            if updated_strs[x+1][y-1] == "*" {
                                count += 1;
                            }

                        },
                        (true, true, true, true) => {
                            if updated_strs[x-1][y] == "*" {
                                count += 1;
                            }
                            if updated_strs[x+1][y] == "*" {
                                count += 1;
                            }
                            if updated_strs[x][y-1] == "*" {
                                count += 1;
                            }
                            if updated_strs[x][y+1] == "*" {
                                count += 1;
                            }

                            if updated_strs[x+1][y+1] == "*" {
                                count += 1;
                            }
                            if updated_strs[x-1][y-1] == "*" {
                                count += 1;
                            }
                            if updated_strs[x-1][y+1] == "*" {
                                count += 1;
                            }
                            if updated_strs[x+1][y-1] == "*" {
                                count += 1;
                            }
                        },
                        (_, _, _, _) => { }
                    }

                    if count > 0 {
                        cleaned_strs[x][y] = count.to_string();
                    }
                }
            }
        }
    }

    cleaned_strs.iter().map(|x| x.join("")).collect()
}
