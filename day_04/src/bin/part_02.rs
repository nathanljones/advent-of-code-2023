use std::thread::current;

fn main() {
    let inputs = include_str!("inputs.txt");
    let sum = count_number_scratchcards(inputs);
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

fn count_number_scratchcards(input: &str) -> u32 {
    let mut current_line_no: u32 = 0;
    let mut scratch_cards: Vec<u32> = Vec::new();

    for line in input.lines() {
        let no_winning_numbers = count_winning_nos_for_line(line);
        if no_winning_numbers == 0 {
            print!("{}", scratch_cards.len());
            if scratch_cards.len() == current_line_no as usize || current_line_no == 0 {
                scratch_cards.push(1);
            } else {
                scratch_cards[current_line_no as usize] =
                    scratch_cards[current_line_no as usize] + 1;
            };
        } else {
            let to_value = no_winning_numbers + current_line_no;
            let mut multipler: u32 = 0;
            if scratch_cards.len() > current_line_no as usize {
                multipler = scratch_cards[current_line_no as usize] + 1;
            } else {
                multipler = 1
            };
            let mut first_pass = true;
            for i in 1..=multipler {
                for line_no in current_line_no..=to_value {
                    if scratch_cards.len() == line_no as usize || current_line_no == 0 {
                        scratch_cards.push(1);
                    } else if first_pass == true && current_line_no == line_no {
                        scratch_cards[line_no as usize] = scratch_cards[line_no as usize] + 1;
                        first_pass = false;
                    } else if first_pass == false && current_line_no == line_no {
                    } else {
                        scratch_cards[line_no as usize] = scratch_cards[line_no as usize] + 1;
                    }
                }
            }
        };
        current_line_no += 1;
    }
    let mut total: u32 = 0;
    let mut smaller_amount: u32 = 0;
    smaller_amount = input.lines().count() as u32;
    if scratch_cards.len() < smaller_amount as usize {
        smaller_amount = scratch_cards.len() as u32;
    };
    for i in 0..=smaller_amount - 1 {
        total += scratch_cards[i as usize];
    }
    total
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
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";
        assert_eq!(count_number_scratchcards(input), 30);
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
}
