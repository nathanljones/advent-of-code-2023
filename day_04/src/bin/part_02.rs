use std::usize;

fn main() {
    let inputs = include_str!("inputs.txt");
    let sum = total_number_of_scratch_cards_won(inputs);
    println!("{sum}");
}

fn count_winning_nos_for_line(line: &str) -> u32 {
    let replaced_line = line.replace(":", "|");
    let split_lines: Vec<&str> = replaced_line.split('|').collect();
    let winning_nos: Vec<u32> = get_card_numbers(&split_lines[1]);
    let numbers_to_find: Vec<u32> = get_card_numbers(&split_lines[2]);

    numbers_to_find
        .iter()
        .filter(|number| winning_nos.contains(number))
        .count() as u32
}
fn get_card_numbers(numbers: &str) -> Vec<u32> {
    numbers
        .split_whitespace()
        .map(|x| x.parse::<u32>().unwrap())
        .collect()
}

fn total_number_of_scratch_cards_won(input: &str) -> u32 {
    let mut cards_won: Vec<u32> = vec![1; input.lines().count()];

    for (current_line, line) in input.lines().enumerate() {
        let line_to: usize = {
            let mut temp_line_to = (count_winning_nos_for_line(line)) as usize + current_line;
            if temp_line_to > input.lines().count() {
                temp_line_to = input.lines().count();
            }
            temp_line_to
        };
        (current_line + 1..=line_to)
            .into_iter()
            .for_each(|value| cards_won[value] += cards_won[current_line]);
    }

    cards_won.iter().sum()
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
        assert_eq!(total_number_of_scratch_cards_won(input), 30);
        //assert_eq!(count_number_points(input), 13);
    }
    #[test]
    fn get_vec_card_numbers() {
        let input = "41 48 83 86 17 ";
        let res = get_card_numbers(input);
        println!("{:?}", res);
    }
    #[test]
    fn number_winning_no_for_card1() {
        let input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53";
        assert_eq!(count_winning_nos_for_line(input), 4)
    }
    #[test]
    fn number_winning_no_for_card2() {
        let input = "Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19";
        assert_eq!(count_winning_nos_for_line(input), 2)
    }
    #[test]
    fn number_winning_no_for_card3() {
        let input = "Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1";
        assert_eq!(count_winning_nos_for_line(input), 2)
    }
    #[test]
    fn number_winning_no_for_card4() {
        let input = "Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83";
        assert_eq!(count_winning_nos_for_line(input), 1)
    }
    #[test]
    fn number_winning_no_for_card5() {
        let input = "Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36";
        assert_eq!(count_winning_nos_for_line(input), 0)
    }
    #[test]
    fn number_winning_no_for_card6() {
        let input = "Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";
        assert_eq!(count_winning_nos_for_line(input), 0)
    }
}
