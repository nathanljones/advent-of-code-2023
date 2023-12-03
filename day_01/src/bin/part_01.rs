fn main() {
    let inputs = include_str!("inputs_01.txt");
    println!("{}", get_sum_for_lines(inputs));
}
fn get_sum_for_lines(lines: &str) -> u32 {
    let mut running_total = 0;
    for line in lines.lines() {
        running_total += get_number_for_line(line);
    }
    running_total
}
fn get_number_for_line(line: &str) -> u32 {
    let mut first_no = 0;
    let mut second_no = 0;
    for character in line.chars() {
        if character.is_numeric() {
            if first_no == 0 {
                first_no = character.to_digit(10).unwrap();
            }
            if first_no != 0 {
                second_no = character.to_digit(10).unwrap();
            }
        }
    }
    if second_no == 0 {
        second_no = first_no;
    }

    first_no * 10 + second_no
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_add() {
        assert_eq!(get_number_for_line("1abc2"), 12);
        assert_eq!(get_number_for_line("pqr3stu8vwx"), 38);
        assert_eq!(get_number_for_line("a1b2c3d4e5f"), 15);
        assert_eq!(get_number_for_line("treb7uchet"), 77);
    }
    #[test]
    fn get_running_total() {
        let lines = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";
        assert_eq!(get_sum_for_lines(lines), 142);
    }
}
