fn main() {
    let inputs = include_str!("inputs.txt");
    let sum = count_number_points(inputs);
    println!("{}", sum);
}

fn count_winning_nos_for_line(line: &str) -> u32 {
    let replaced_line = line.replace(":", "|");
    let split_lines: Vec<&str> = replaced_line.split('|').collect();
    let mut count: u32 = 0;
    let winning_nos: Vec<u32> = get_card_numbers(&split_lines[1]);
    let numbers_to_find: Vec<u32> = get_card_numbers(&split_lines[2]);

    for number in winning_nos {
        if numbers_to_find.contains(&number) {
            count += 1;
        }
    }
    count
}
fn get_card_numbers(numbers: &str) -> Vec<u32> {
    let ret_val: Vec<u32> = numbers
        .split_whitespace()
        .map(|x| x.parse::<u32>().unwrap())
        .collect();
    ret_val
}

fn calculate_no_points(number: u32) -> u32 {
    if number > 0 {
        let power: u32 = number - 1;
        2_u32.pow(power)
    } else {
        0
    }
}
fn count_number_points(input: &str) -> u32 {
    let mut running_total: u32 = 0;
    for line in input.lines() {
        let points_for_line = count_winning_nos_for_line(line);
        running_total = running_total + calculate_no_points(points_for_line);
    }
    running_total
}
#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn get_card_totals() {
        let input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";
        assert_eq!(count_number_points(input), 13);
    }
    #[test]
    fn get_vec_card_numbers() {
        let input = "41 48 83 86 17 ";
        let res = get_card_numbers(input);
        println!("{:?}", res);
    }
    #[test]
    fn number_winning_no_for_line() {
        let input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53";
        assert_eq!(count_winning_nos_for_line(input), 4)
    }
    #[test]
    fn calculate_powers() {
        assert_eq!(calculate_no_points(2), 2);
        assert_eq!(calculate_no_points(4), 8);
        assert_eq!(calculate_no_points(0), 0);
    }
}
