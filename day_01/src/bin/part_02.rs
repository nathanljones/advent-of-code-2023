struct FindChars {
    string: String,
    number: u32,
}
fn main() {
    let inputs = include_str!("inputs_01.txt");
    println!("{}", get_sum_for_lines(inputs));
}
fn get_sum_for_lines(lines: &str) -> u32 {
    lines
        .lines()
        .into_iter()
        .map(|line| get_number_for_line(line))
        .sum()
}
fn fill_find_structure() -> Vec<FindChars> {
    let mut ret_vec: Vec<FindChars> = vec![];
    ret_vec.push(FindChars {
        string: (String::from("1")),
        number: (1),
    });

    ret_vec.push(FindChars {
        string: (String::from("one")),
        number: (1),
    });

    ret_vec.push(FindChars {
        string: (String::from("2")),
        number: (2),
    });

    ret_vec.push(FindChars {
        string: (String::from("two")),
        number: (2),
    });

    ret_vec.push(FindChars {
        string: (String::from("3")),
        number: (3),
    });
    ret_vec.push(FindChars {
        string: (String::from("three")),
        number: (3),
    });
    ret_vec.push(FindChars {
        string: (String::from("4")),
        number: (4),
    });

    ret_vec.push(FindChars {
        string: (String::from("four")),
        number: (4),
    });

    ret_vec.push(FindChars {
        string: (String::from("5")),
        number: (5),
    });

    ret_vec.push(FindChars {
        string: (String::from("five")),
        number: (5),
    });

    ret_vec.push(FindChars {
        string: (String::from("6")),
        number: (6),
    });

    ret_vec.push(FindChars {
        string: (String::from("six")),
        number: (6),
    });

    ret_vec.push(FindChars {
        string: (String::from("7")),
        number: (7),
    });

    ret_vec.push(FindChars {
        string: (String::from("seven")),
        number: (7),
    });

    ret_vec.push(FindChars {
        string: (String::from("eight")),
        number: (8),
    });

    ret_vec.push(FindChars {
        string: (String::from("8")),
        number: (8),
    });

    ret_vec.push(FindChars {
        string: (String::from("9")),
        number: (9),
    });

    ret_vec.push(FindChars {
        string: (String::from("nine")),
        number: (9),
    });
    ret_vec
}
fn get_number_for_line(line: &str) -> u32 {
    let mut first_no = 0;
    let mut second_no = 0;

    let mut first_pos = 0;
    let mut second_pos = 0;

    let chars_to_find = fill_find_structure();

    for find_chars in chars_to_find {
        if let Some(char_no) = line.find(find_chars.string.as_str()) {
            if char_no > 0 && first_no == 0 {
                first_pos = char_no;
                first_no = find_chars.number;
            }
            if first_pos >= char_no {
                first_pos = char_no;
                first_no = find_chars.number;
            }
        }
        if let Some(char_no) = line.rfind(find_chars.string.as_str()) {
            if second_pos <= char_no {
                second_pos = char_no;
                second_no = find_chars.number;
            }
        }
    }

    first_no * 10 + second_no
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_add() {
        assert_eq!(get_number_for_line("two1nine"), 29);
        assert_eq!(get_number_for_line("eightwothree"), 83);
        assert_eq!(get_number_for_line("abcone2threexyz"), 13);
        assert_eq!(get_number_for_line("xtwone3four"), 24);
        assert_eq!(get_number_for_line("4nineeightseven2"), 42);
        assert_eq!(get_number_for_line("zoneight234"), 14);
        assert_eq!(get_number_for_line("7pqrstsixteen"), 76);
    }
}
