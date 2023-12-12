#[derive(Debug, Copy, Clone)]
struct Document {
    time: u32,
    distance: u32,
}
fn main() {
    let inputs = include_str!("input.txt");
    println!("{}", main_process(inputs));
}
fn main_process(input: &str) -> u32 {
    let documents = parse_input(input);
    let mut ret_val: u32 = 1;
    for document in documents {
        let no_ways = count_no_ways(document);
        if no_ways > 0 {
            ret_val *= no_ways;
        }
    }
    ret_val
}
fn count_no_ways(document: Document) -> u32 {
    let mut combo_count: u32 = 0;
    for button_hold in 0..=document.time {
        let travel: u32 = button_hold * (document.time - button_hold);
        if travel > document.distance {
            combo_count += 1;
        }
    }
    combo_count
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

    for (count, time) in times.into_iter().enumerate() {
        ret_vec.push(Document {
            time,
            distance: distances[count],
        });
    }
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
        assert_eq!(main_process(input), 288);
    }
    #[test]
    fn test_parse_input() {
        let input = "Time:      7  15   30
Distance:  9  40  200";
        let result = parse_input(input);
        println!("{:?}", result);
    }

    #[test]
    fn test_count_ways() {
        let input = "Time:      7  15   30
Distance:  9  40  200";
        let result = parse_input(input);
        assert_eq!(count_no_ways(result[0]), 4);
        assert_eq!(count_no_ways(result[1]), 8);
        assert_eq!(count_no_ways(result[2]), 9);
    }
    #[test]
    fn test_main_processing() {
        let input = "Time:      7  15   30
Distance:  9  40  200";
        assert_eq!(main_process(input), 288);
    }
}
