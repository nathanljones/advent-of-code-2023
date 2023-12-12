fn main() {
    let inputs = include_str!("inputs.txt");
    let sum = get_gear_ratios(inputs);
    println!("{}", sum);
}

fn get_gear_ratios(input: &str) -> u32 {
    let max_no_lines = input.lines().count() - 1;
    let mut line_pointer: u32 = 0;
    let mut current_number = 0;
    let mut total = 0;
    let mut start_of_number = 0;
    let mut end_of_number = 0;
    let mut current_line_number = 0;
    let mut does_symbol_exist: bool = false;

    for line in input.lines() {
        while line_pointer < line.len() as u32 {
            current_number = find_no(line, line_pointer);
            start_of_number = get_start_of_number(line, line_pointer);
            end_of_number = get_end_of_number(line, line_pointer);
            line_pointer = end_of_number;
            if current_line_number == 0 {
                does_symbol_exist =
                    does_symbol_exist_on_line_in_range(line, start_of_number, end_of_number);
                if does_symbol_exist == false {
                    does_symbol_exist = does_symbol_exist_on_line_in_range(
                        input.lines().nth(current_line_number + 1).unwrap(),
                        start_of_number,
                        end_of_number,
                    );
                }
                if does_symbol_exist == true {
                    total = total + current_number;
                }
            } else if current_line_number == max_no_lines {
                does_symbol_exist =
                    does_symbol_exist_on_line_in_range(line, start_of_number, end_of_number);
                if does_symbol_exist == false {
                    does_symbol_exist = does_symbol_exist_on_line_in_range(
                        input.lines().nth(current_line_number - 1).unwrap(),
                        start_of_number,
                        end_of_number,
                    );
                }
                if does_symbol_exist == true {
                    total = total + current_number;
                }
            } else {
                does_symbol_exist =
                    does_symbol_exist_on_line_in_range(line, start_of_number, end_of_number);
                if does_symbol_exist == false {
                    does_symbol_exist = does_symbol_exist_on_line_in_range(
                        input.lines().nth(current_line_number + 1).unwrap(),
                        start_of_number,
                        end_of_number,
                    );
                }
                if does_symbol_exist == false {
                    does_symbol_exist = does_symbol_exist_on_line_in_range(
                        input.lines().nth(current_line_number - 1).unwrap(),
                        start_of_number,
                        end_of_number,
                    );
                }
                if does_symbol_exist == true {
                    total = total + current_number;
                }
            }
        }

        current_line_number += 1;
        line_pointer = 0;
    }
    // dfor line in input.lines() {}
    total
}
fn does_symbol_exist_on_line_in_range(line: &str, start_pos: u32, end_pos: u32) -> bool {
    let mut num_start_pos = 0;
    let mut num_end_pos = 0;
    if start_pos == 0 {
        num_start_pos = (start_pos) as usize;
    } else {
        num_start_pos = (start_pos - 1) as usize;
    }
    if end_pos == (line.len()) as u32 {
        num_end_pos = (end_pos) as usize;
    } else {
        num_end_pos = (end_pos + 1) as usize;
    }
    let cleansed_line = line.replace(".", " ");
    let trimmed_string = cleansed_line.get(num_start_pos..num_end_pos).unwrap();
    let mut ret_val = false;
    for char in trimmed_string.to_string().chars() {
        if char == '*' {
            ret_val = true
        }
    }

    if ret_val == true {
        ret_val
    } else {
        false
    }
}
fn find_no(line: &str, start: u32) -> u32 {
    let mut count = 0;
    let mut number_chars = String::from("");
    let mut ret_number = String::from("");
    let mut ret_value: u32 = 0;
    for character in line.chars() {
        if count >= start {
            if character.is_numeric() {
                number_chars.push(character);
            } else if (character.is_numeric() == false) && number_chars.chars().count() > 0 {
                break;
            }
        }
        count += 1;
    }
    for number in number_chars.chars() {
        ret_number.push_str(&number.to_string());
    }
    if ret_number != "" {
        ret_value = ret_number.parse::<u32>().unwrap();
    }
    ret_value
}
fn get_start_of_number(line: &str, start: u32) -> u32 {
    let mut position: u32 = 0;
    for character in line.chars() {
        if position >= start {
            if character.is_numeric() {
                break;
            }
        }
        position += 1;
    }
    position
}
fn get_end_of_number(line: &str, start: u32) -> u32 {
    let mut position: u32 = 0;
    let mut number_found: bool = false;
    for character in line.chars() {
        if position >= start {
            if character.is_numeric() {
                number_found = true;
            }
            if character.is_numeric() == false && number_found == true {
                break;
            }
        }
        position += 1;
    }
    position
}
#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn get_test_value() {
        let test_string = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";
        assert_eq!(get_gear_ratios(test_string), 467835);
    }
    #[test]
    fn find_number() {
        assert_eq!(find_no("467..114..", 0), 467);
        assert_eq!(find_no("467..114..", 4), 114);
    }

    #[test]
    fn start_of_number() {
        assert_eq!(get_start_of_number("467..114..", 0), 0);
        assert_eq!(get_start_of_number("467..114..", 4), 5);
    }
    #[test]
    fn end_of_number() {
        assert_eq!(get_end_of_number("467..114..", 0), 3);
        assert_eq!(get_end_of_number("467..114..", 4), 8);
    }
    #[test]
    fn symbol_in_range() {
        assert_eq!(does_symbol_exist_on_line_in_range("...*......", 0, 6), true);
        assert_eq!(
            does_symbol_exist_on_line_in_range("...*......", 0, 2),
            false
        );
    }
}
