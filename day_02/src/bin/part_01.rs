fn main() {
    let mut sum = 0;
    let inputs = include_str!("inputs.txt");
    for line in inputs.lines() {
        if is_line_possible(line, 12, 13, 14) {
            sum += get_game_number(line);
        }
    }
    println!("{}", sum);
}
fn get_game_number(line: &str) -> u32 {
    let lines: Vec<&str> = line.split(':').collect();
    let game_line = lines[0];
    let space_pos = game_line.find(' ').unwrap() + 1;
    let ret_no = game_line[space_pos..game_line.len()].to_string();
    ret_no.parse::<u32>().unwrap()
}

fn is_line_possible(line: &str, no_red: u32, no_green: u32, no_blue: u32) -> bool {
    let max_red = get_max_colour(line, "red");
    let max_blue: u32 = get_max_colour(line, "blue");
    let max_green: u32 = get_max_colour(line, "green");

    if max_red > no_red || max_blue > no_blue || max_green > no_green {
        false
    } else {
        true
    }
}
fn get_max_colour(line: &str, colour: &str) -> u32 {
    let replaced_line = line.replace(";", ",");
    let game_lines_split: Vec<&str> = replaced_line.split(':').collect();
    let colour_lines: Vec<&str> = game_lines_split[1].split(',').collect();
    let mut max_number = 0;
    for split_line in colour_lines {
        if let Some(colour_pos) = split_line.find(colour) {
            let number = split_line[..colour_pos].to_string();
            let actual_number: u32 = number.trim().parse::<u32>().unwrap();
            if max_number < actual_number {
                max_number = actual_number
            }
        }
    }
    max_number
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_add() {
        let lest_string = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
    }
    #[test]
    fn get_game_nos() {
        assert_eq!(
            get_game_number("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green"),
            1
        );
        assert_eq!(
            get_game_number("Game 10: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green"),
            10
        );
        assert_eq!(
            get_game_number("Game 100: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green"),
            100
        );
        assert_eq!(
            get_game_number("Game 1000: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green"),
            1000
        );
    }
    #[test]
    fn get_max_colours() {
        assert_eq!(
            get_max_colour(
                "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 12 green",
                "green"
            ),
            12
        );
        assert_eq!(
            get_max_colour(
                "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green",
                "blue"
            ),
            6
        );
        assert_eq!(
            get_max_colour(
                "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green",
                "red"
            ),
            4
        );
        assert_eq!(
            get_max_colour(
                "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green",
                "nathan"
            ),
            0
        );
    }
    #[test]
    fn is_valid_line() {
        assert_eq!(
            is_line_possible(
                "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green",
                12,
                13,
                14
            ),
            true
        );
        assert_eq!(
            is_line_possible(
                "Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue",
                12,
                13,
                14
            ),
            true
        );
        assert_eq!(
            is_line_possible(
                "Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red",
                12,
                13,
                14
            ),
            false
        );
        assert_eq!(
            is_line_possible(
                "Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red",
                12,
                13,
                14
            ),
            false
        );
        assert_eq!(
            is_line_possible(
                "Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green",
                12,
                13,
                14
            ),
            true
        );
    }
}
