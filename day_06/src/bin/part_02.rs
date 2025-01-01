#[derive(Debug, Copy, Clone)]
struct Document {
    time: u32,
    distance: u64,
}
fn main() {
    let inputs = include_str!("input.txt");
    println!("{}", main_process(inputs));
}
fn main_process(input: &str) -> u64 {
    let documents = parse_input(input);
    let mut ret_val: u64 = 1;
    for document in documents {
        let min_no_ways = count_min_no_ways(document);
        let max_no_ways = count_max_no_ways(document);
        ret_val = max_no_ways - min_no_ways + 1;
    }
    ret_val
}
fn count_min_no_ways(document: Document) -> u64 {
    let mut combo_count: u64 = 0;
    let mut ret_value: u64 = 0;
    for button_hold in 0..=document.time {
        let temp = (document.time - button_hold) as u128;
        println!(
            "document time = {}, button_hold = {}, temp = {}",
            document.time, button_hold, temp
        );

        let travel: u128 = button_hold as u128 * temp;
        if travel > document.distance.into() {
            ret_value = button_hold as u64;
            break;
        }
    }
    ret_value
}
fn count_max_no_ways(document: Document) -> u64 {
    let mut combo_count: u64 = 0;
    let mut ret_value: u64 = 0;
    for button_hold in (0..=document.time).rev() {
        let temp = (document.time - button_hold) as u128;
        println!(
            "document time = {}, button_hold = {}, temp = {}",
            document.time, button_hold, temp
        );

        let travel: u128 = button_hold as u128 * temp;
        if travel >= document.distance.into() && travel != 0 {
            ret_value = button_hold as u64;
            break;
        }
    }
    ret_value
}

fn parse_input(input: &str) -> Vec<Document> {
    let lines: Vec<&str> = input.lines().collect();
    let time_line = lines[0];
    let distance_line = lines[1];

    let time_line_nos: Vec<&str> = time_line.split(':').collect();
    let distance_nos: Vec<&str> = distance_line.split(':').collect();

    let times: Vec<u32> = time_line_nos[1]
        .split_whitespace()
        .map(|x| x.parse::<u32>().unwrap())
        .collect();
    let distances: Vec<u32> = distance_nos[1]
        .split_whitespace()
        .map(|x| x.parse::<u32>().unwrap())
        .collect();
    let mut ret_vec: Vec<Document> = Vec::new();
    let mut times_no_string: String = String::new();
    for number in times {
        let string_no = number.to_string();
        times_no_string.push_str(&string_no);
    }
    let mut distance_no_string: String = String::new();
    for number in distances {
        let string_no = number.to_string();
        distance_no_string.push_str(&string_no);
    }

    let mut ret_vec: Vec<Document> = Vec::new();
    ret_vec.push(Document {
        time: times_no_string.parse::<u32>().unwrap(),
        distance: distance_no_string.parse::<u64>().unwrap(),
    });
    ret_vec
}
#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_process() {
        let input = "Time:      7  15   30
Distance:  9  40  200";
        assert_eq!(main_process(input), 71503);
    }
    #[test]
    fn test_min_no_ways() {
        let input = "Time:      7  15   30
Distance:  9  40  200";
        let documents = parse_input(input);
        assert_eq!(count_min_no_ways(documents[0]), 14);
    }
    #[test]
    fn test_max_no_ways() {
        let input = "Time:      7  15   30
Distance:  9  40  200";
        let documents = parse_input(input);
        assert_eq!(count_max_no_ways(documents[0]), 71516);
    }
}
