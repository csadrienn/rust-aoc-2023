use std::{cmp::Ordering, collections::HashSet, str::FromStr};

fn main() {
    let input = include_str!("input.txt");
    let res_part_1 = part_1(input);
    println!("{}", res_part_1);
    let res_part_2 = part_2(input);
    println!("{}", res_part_2);
}

#[derive(Debug)]
enum HighCard {
    A,
    K,
    Q,
    J,
    T,
}

impl HighCard {
    fn get_value(&self, with_joker: bool) -> u32 {
        match self {
            HighCard::A => 14,
            HighCard::K => 13,
            HighCard::Q => 12,
            HighCard::J if with_joker => 1,
            HighCard::J => 11,
            HighCard::T => 10,
        }
    }
}

impl FromStr for HighCard {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" => Ok(HighCard::A),
            "K" => Ok(HighCard::K),
            "Q" => Ok(HighCard::Q),
            "J" => Ok(HighCard::J),
            "T" => Ok(HighCard::T),
            _ => Err("Invalid value"),
        }
    }
}

#[derive(Debug)]
enum Hand {
    FiveOfKind,
    FourOfKind,
    FullHouse,
    ThreeOfKind,
    TwoPair,
    OnePair,
    HighCard,
}

impl Hand {
    fn get_value(&self) -> u32 {
        match self {
            Hand::HighCard => 1,
            Hand::OnePair => 2,
            Hand::TwoPair => 3,
            Hand::ThreeOfKind => 4,
            Hand::FullHouse => 5,
            Hand::FourOfKind => 6,
            Hand::FiveOfKind => 7,
        }
    }

    fn parse_from_string(s: &str) -> Option<Hand> {
        let char_set: HashSet<char> = s.chars().collect();
        let mut chars: Vec<char> = s.chars().collect();
        chars.sort();

        match char_set.len() {
            1 => Some(Hand::FiveOfKind),
            2 if has_same_chars(&chars, 4) => Some(Hand::FourOfKind),
            2 => Some(Hand::FullHouse),
            3 if has_same_chars(&chars, 3) => Some(Hand::ThreeOfKind),
            3 => Some(Hand::TwoPair),
            4 => Some(Hand::OnePair),
            5 => Some(Hand::HighCard),
            _ => None,
        }
    }

    fn parse_from_string_with_joker(s: &str) -> Option<Hand> {
        let jokers: Vec<_> = s.chars().filter(|c| c == &'J').collect();
        if jokers.len() == 0 || jokers.len() == 5 {
            return Self::parse_from_string(s);
        } else {
            let mut hand_without_jokers: Vec<_> = s.chars().filter(|&c| c != 'J').collect();
            let char_set: HashSet<_> = hand_without_jokers.iter().collect();
            let char_set_len = char_set.len();
            match jokers.len() {
                1 => match char_set_len {
                    1 => Some(Hand::FiveOfKind),
                    2 => {
                        hand_without_jokers.sort();
                        if has_same_chars(&hand_without_jokers, 3) {
                            return Some(Hand::FourOfKind);
                        }
                        return Some(Hand::FullHouse);
                    }
                    3 => Some(Hand::ThreeOfKind),
                    _ => Some(Hand::OnePair),
                },
                2 => match char_set_len {
                    1 => Some(Hand::FiveOfKind),
                    2 => Some(Hand::FourOfKind),
                    _ => Some(Hand::ThreeOfKind),
                },
                3 => match char_set_len {
                    1 => Some(Hand::FiveOfKind),
                    _ => Some(Hand::FourOfKind),
                },
                _ => Some(Hand::FiveOfKind),
            }
        }
    }
}

fn has_same_chars(chars: &Vec<char>, size: usize) -> bool {
    chars.windows(size).any(|w| w.iter().all(|&c| c == w[0]))
}

fn part_1(input: &str) -> usize {
    let mut lines: Vec<_> = input
        .lines()
        .map(|line| line.split_once(' ').unwrap())
        .collect();

    lines.sort_by(|a, b| compare_hands(a.0, b.0));

    lines
        .into_iter()
        .enumerate()
        .map(|(i, (_, bid))| bid.parse::<usize>().unwrap() * (i + 1))
        .sum()
}

fn part_2(input: &str) -> usize {
    let mut lines: Vec<_> = input
        .lines()
        .map(|line| line.split_once(' ').unwrap())
        .collect();

    lines.sort_by(|a, b| compare_hands_with_joker(a.0, b.0));

    lines
        .into_iter()
        .enumerate()
        .map(|(i, (_, bid))| bid.parse::<usize>().unwrap() * (i + 1))
        .sum()
}

fn compare_hands_with_joker(hand_a: &str, hand_b: &str) -> Ordering {
    let hand_value_a = Hand::parse_from_string_with_joker(hand_a)
        .unwrap()
        .get_value();
    let hand_value_b = Hand::parse_from_string_with_joker(hand_b)
        .unwrap()
        .get_value();

    let ordering = hand_value_a.cmp(&hand_value_b);
    if ordering == Ordering::Equal {
        return compare_cards(hand_a, hand_b, true);
    }
    ordering
}

fn compare_hands(hand_a: &str, hand_b: &str) -> Ordering {
    let hand_value_a = Hand::parse_from_string(hand_a).unwrap().get_value();
    let hand_value_b = Hand::parse_from_string(hand_b).unwrap().get_value();

    let ordering = hand_value_a.cmp(&hand_value_b);
    if ordering == Ordering::Equal {
        return compare_cards(hand_a, hand_b, false);
    }
    ordering
}

fn compare_cards(hand_a: &str, hand_b: &str, with_joker: bool) -> Ordering {
    let chars_b: Vec<_> = hand_b.chars().collect();
    let mut ordering = Ordering::Equal;

    for (i, c) in hand_a.chars().enumerate() {
        let num_a: u32 = parse_hand_char(c, with_joker);
        let num_b: u32 = parse_hand_char(chars_b[i], with_joker);

        ordering = num_a.cmp(&num_b);
        if ordering != Ordering::Equal {
            break;
        }
    }

    ordering
}

fn parse_hand_char(c: char, with_joker: bool) -> u32 {
    c.to_digit(10).unwrap_or_else(|| {
        HighCard::from_str(&c.to_string())
            .unwrap()
            .get_value(with_joker)
    })
}

#[cfg(test)]
mod tests {
    use crate::{part_1, part_2};

    #[test]
    fn calculates_part_1_correctly() {
        let input = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";
        let actual = part_1(input);
        let expected = 6440;
        assert_eq!(actual, expected)
    }

    #[test]
    fn calculates_part_2_correctly() {
        let input = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";
        let actual = part_2(input);
        let expected = 5905;
        assert_eq!(actual, expected)
    }
}
