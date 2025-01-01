struct Node {
    label: String,
    left: String,
    right: String,
}
fn main() {
    let inputs = include_str!("input.txt");
    println!("{}", main_process(inputs));
}
fn main_process(input: &str) -> u32 {
    let directions = parse_directions(input.lines().next().unwrap());
    let mut nodes: Vec<Node> = Vec::new();
    for (loop_pos, line) in input.lines().enumerate() {
        if loop_pos > 1 {
            let node = parse_node(line);
            nodes.push(node);
        };
    }

    traverse_nodes(&directions, &nodes)
}

fn parse_directions(directions: &str) -> Vec<char> {
    directions.chars().collect()
}
fn traverse_nodes(directions: &Vec<char>, nodes: &Vec<Node>) -> u32 {
    let mut label = "AAA".to_string();
    let mut count: u32 = 0;
    while label != "ZZZ" {
        for instruction in directions {
            label = find_next_label(&label, nodes, &instruction.to_string());
            count += 1;
        }
    }

    count
}

fn find_next_label(label: &str, nodes: &Vec<Node>, instruction: &str) -> String {
    let mut ret_val = String::new();
    for node in nodes {
        if node.label == *label {
            match instruction {
                "L" => ret_val = node.left.to_owned(),
                "R" => ret_val = node.right.to_owned(),
                _ => ret_val = "".to_string(),
            }
            break;
        }
    }
    ret_val
}
fn parse_node(input: &str) -> Node {
    let label = input.split('=').next().unwrap().trim();
    let mut left = input.split('(').last().unwrap();
    left = left.split(',').next().unwrap().trim();
    let mut right = input.split_ascii_whitespace().last().unwrap();
    right = right.split(')').next().unwrap().trim();

    Node {
        label: label.to_string(),
        left: left.to_string(),
        right: right.to_string(),
    }
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_add() {
        let input1 = "RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)";
        let input2 = "LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)";
        assert_eq!(main_process(input1), 2);
        assert_eq!(main_process(input2), 6);
    }
}
