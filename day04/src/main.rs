use std::collections::HashSet;

fn main() {
    let input = include_str!("input.txt");
    let res_part_1 = part_1(input);
    let res_part_2 = part_2(input);
    println!("{}", res_part_1);
    println!("{}", res_part_2);
}

fn part_1(input: &str) -> i32 {
    input
        .lines()
        .map(|line| {
            let matching = extract_matching_numbers(line);
            get_game_points(matching)
        })
        .sum()
}

fn part_2(input: &str) -> i32 {
    let card_wins = input.lines().map(|line| {
        let matching = extract_matching_numbers(line);
        matching.len()
    });
    let mut all_cards: Vec<i32> = vec![1; card_wins.clone().count()];
    card_wins.enumerate().for_each(|(i, wins)| {
        let card_copies = all_cards[i];
        for n in 1..wins + 1 {
            if i + n < all_cards.len() {
                all_cards[i + n] += card_copies
            }
        }
    });
    all_cards.iter().sum()
}

fn extract_matching_numbers(line: &str) -> Vec<i32> {
    let (_, card_data) = line.split_once(':').unwrap();
    let (winning_part, numbers_part) = card_data.split_once('|').unwrap();
    let winning_numbers = get_numbers(winning_part);
    let numbers = get_numbers(numbers_part);
    let matching: Vec<i32> = winning_numbers.intersection(&numbers).copied().collect();
    matching
}

fn get_numbers(numbers_snippet: &str) -> HashSet<i32> {
    numbers_snippet
        .trim()
        .split(' ')
        .filter_map(|n| n.parse().ok())
        .collect()
}

fn get_game_points(matching: Vec<i32>) -> i32 {
    if matching.len() < 2 {
        return matching.len() as i32;
    }
    matching.into_iter().skip(1).fold(1, |acc, _| acc * 2)
}
#[cfg(test)]
mod tests {
    use crate::{part_1, part_2};

    #[test]
    fn calculates_part_1_correctly() {
        let input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";
        let actual = part_1(input);
        let expected = 13;
        assert_eq!(actual, expected)
    }

    #[test]
    fn calculates_part_2_correctly() {
        let input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";
        let actual = part_2(input);
        let expected = 30;
        assert_eq!(actual, expected)
    }
}
