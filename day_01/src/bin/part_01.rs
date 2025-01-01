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
fn get_number_for_line(line: &str) -> u32 {
    let numbers_as_string: Vec<char> = line.chars().filter(|char| char.is_numeric()).collect();

    let first_number = numbers_as_string.first().unwrap().to_digit(10).unwrap();
    let second_number = numbers_as_string.last().unwrap().to_digit(10).unwrap();
    first_number * 10 + second_number
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
