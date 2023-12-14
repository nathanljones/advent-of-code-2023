#[derive(PartialEq, Debug)]
enum HandType {
    FiveOfAKind,
    FourOfAKind,
    FullHouse,
    ThreeOfAKind,
    TwoPair,
    OnePair,
    HighCard,
}

#[derive(Clone, Debug)]
struct HandRank {
    hand: String,
    rank: u32,
}
fn main() {
    let inputs = include_str!("input.txt");
    println!("{}", main_process(inputs));
}

fn main_process(input: &str) -> u32 {
    let mut ranked_hands: Vec<HandRank> = Vec::new();
    for line in input.lines() {
        let hand_rank = HandRank {
            hand: line.split_ascii_whitespace().next().unwrap().to_string(),
            rank: line
                .split_ascii_whitespace()
                .nth(1)
                .unwrap()
                .parse::<u32>()
                .unwrap(),
        };
        if ranked_hands.is_empty() {
            ranked_hands.push(hand_rank);
        } else {
            let mut ranked_hands_copy = ranked_hands.clone();
            let mut is_found = false;
            for (loop_position, ranked_hand) in ranked_hands.iter().enumerate() {
                let temp: &str = &hand_rank.hand;
                if is_first_hand_higher(&ranked_hand.hand, temp) == true {
                    ranked_hands_copy.insert(loop_position, hand_rank.to_owned());
                    is_found = true;
                    break;
                }
            }
            if is_found != true {
                ranked_hands_copy.push(hand_rank);
            }
            ranked_hands = ranked_hands_copy.to_owned();
        }
    }
    println!("{:?}", ranked_hands);
    let mut ret_val: u32 = 0;
    for (loop_position, hand) in ranked_hands.iter().enumerate() {
        ret_val += hand.rank * (loop_position as u32 + 1);
    }
    ret_val
}
fn get_hand_type(hand: &str) -> HandType {
    let mut individual_cards: Vec<char> = hand.chars().collect();
    individual_cards.sort();
    let ret_hand_type: HandType;
    if individual_cards[0] == individual_cards[4] {
        ret_hand_type = HandType::FiveOfAKind;
    } else if (individual_cards[0] == individual_cards[3])
        || (individual_cards[1] == individual_cards[4])
    {
        ret_hand_type = HandType::FourOfAKind;
    } else if (individual_cards[0] == individual_cards[1]
        && individual_cards[2] == individual_cards[4])
        || (individual_cards[0] == individual_cards[2]
            && individual_cards[3] == individual_cards[4])
    {
        ret_hand_type = HandType::FullHouse;
    } else if (individual_cards[0] != individual_cards[1]
        && individual_cards[2] == individual_cards[4])
        || (individual_cards[0] == individual_cards[2]
            && individual_cards[3] != individual_cards[4])
    {
        ret_hand_type = HandType::ThreeOfAKind;
    } else if is_hand_a_two_pair(&individual_cards) {
        ret_hand_type = HandType::TwoPair
    } else if is_hand_a_one_pair(&individual_cards) {
        ret_hand_type = HandType::OnePair
    } else {
        ret_hand_type = HandType::HighCard
    }

    ret_hand_type
}

fn is_hand_a_two_pair(hand: &Vec<char>) -> bool {
    let mut count: u32 = 0;
    for (loop_pos, charactor) in hand.iter().enumerate() {
        if loop_pos + 1 < hand.len() && (charactor == &hand[loop_pos + 1]) {
            count += 1;
        }
    }

    count == 2
}

fn is_hand_a_one_pair(hand: &Vec<char>) -> bool {
    let mut count: u32 = 0;
    for (loop_pos, charactor) in hand.iter().enumerate() {
        if loop_pos + 1 < hand.len() && (charactor == &hand[loop_pos + 1]) {
            count += 1;
        }
    }

    count == 1
}

fn get_card_strength(card: &char) -> u32 {
    match card {
        'A' => 13,
        'K' => 12,
        'Q' => 11,
        'J' => 10,
        'T' => 9,
        '9' => 8,
        '8' => 7,
        '7' => 6,
        '6' => 5,
        '5' => 4,
        '4' => 3,
        '3' => 2,
        '2' => 1,
        _ => 0,
    }
}

fn is_first_hand_higher(first_hand: &str, second_hand: &str) -> bool {
    if get_hand_type(first_hand) == get_hand_type(second_hand) {
        for (loop_position, first_hand_card) in first_hand.chars().enumerate() {
            if get_card_strength(&first_hand_card)
                == get_card_strength(&second_hand.chars().nth(loop_position).unwrap())
            {
                continue;
            } else if get_card_strength(&first_hand_card)
                > get_card_strength(&second_hand.chars().nth(loop_position).unwrap())
            {
                return true;
            } else if get_card_strength(&first_hand_card)
                < get_card_strength(&second_hand.chars().nth(loop_position).unwrap())
            {
                return false;
            }
        }
    }
    if get_hand_rank(get_hand_type(first_hand)) > get_hand_rank(get_hand_type(second_hand)) {
        return true;
    }
    get_hand_rank(get_hand_type(first_hand)) >= get_hand_rank(get_hand_type(second_hand))
}

fn get_hand_rank(hand: HandType) -> u32 {
    match hand {
        HandType::FiveOfAKind => 7,
        HandType::FourOfAKind => 6,
        HandType::FullHouse => 5,
        HandType::ThreeOfAKind => 4,
        HandType::TwoPair => 3,
        HandType::OnePair => 2,
        HandType::HighCard => 1,
    }
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_get_correct_hand_type() {
        assert_eq!(get_hand_type("AAAAA"), HandType::FiveOfAKind);
        assert_eq!(get_hand_type("AA8AA"), HandType::FourOfAKind);
        assert_eq!(get_hand_type("23332"), HandType::FullHouse);
        assert_eq!(get_hand_type("TTT98"), HandType::ThreeOfAKind);
        assert_eq!(get_hand_type("23432"), HandType::TwoPair);
        assert_eq!(get_hand_type("A23A4"), HandType::OnePair);
        assert_eq!(get_hand_type("23456"), HandType::HighCard);
        assert_eq!(get_hand_type("33332"), HandType::FourOfAKind);
        assert_eq!(get_hand_type("2AAAA"), HandType::FourOfAKind);
        assert_eq!(get_hand_type("77888"), HandType::FullHouse);
        assert_eq!(get_hand_type("77788"), HandType::FullHouse);
        assert_eq!(get_hand_type("32T3K"), HandType::OnePair);
        assert_eq!(get_hand_type("KK677"), HandType::TwoPair);
        assert_eq!(get_hand_type("KTJJT"), HandType::TwoPair);
        assert_eq!(get_hand_type("T55J5"), HandType::ThreeOfAKind);
        assert_eq!(get_hand_type("QQQJA"), HandType::ThreeOfAKind);
    }

    #[test]
    fn test_hand_rank() {
        assert_eq!(get_hand_rank(HandType::FiveOfAKind), 7);
        assert_eq!(get_hand_rank(HandType::FourOfAKind), 6);
        assert_eq!(get_hand_rank(HandType::FullHouse), 5);
        assert_eq!(get_hand_rank(HandType::ThreeOfAKind), 4);
        assert_eq!(get_hand_rank(HandType::TwoPair), 3);
        assert_eq!(get_hand_rank(HandType::OnePair), 2);
        assert_eq!(get_hand_rank(HandType::HighCard), 1);
    }

    #[test]
    fn test_is_first_hand_higher() {
        assert!(is_first_hand_higher("33332", "2AAAA"));
        assert!(is_first_hand_higher("77888", "77788"));
        assert_eq!(is_first_hand_higher("2AAAA", "33332"), false);
        assert_eq!(is_first_hand_higher("77788", "77888"), false);
    }
    #[test]
    fn test_main_process() {
        let input = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";
        assert_eq!(main_process(input), 6440);
    }
}
