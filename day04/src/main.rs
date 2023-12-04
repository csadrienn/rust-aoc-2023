use std::collections::HashSet;

fn main() {
    let input = include_str!("input.txt");
    let sum_part_1 = part_1(input);
    println!("{}", sum_part_1);
}

fn part_1(input: &str) -> i32 {
    input
        .lines()
        .map(|line| {
            let (_, card_data) = line.split_once(':').unwrap();
            let (winning_part, numbers_part) = card_data.split_once('|').unwrap();
            let winning_numbers = get_numbers(winning_part);
            let numbers = get_numbers(numbers_part);
            let matching = winning_numbers
                .intersection(&numbers)
                .collect::<Vec<&i32>>();
            get_game_points(matching)
        })
        .sum()
}

fn get_numbers(numbers_snippet: &str) -> HashSet<i32> {
    numbers_snippet
        .trim()
        .split(' ')
        .filter_map(|n| n.parse().ok())
        .collect()
}

fn get_game_points(matching: Vec<&i32>) -> i32 {
    if matching.len() < 2 {
        return matching.len() as i32;
    }
    matching.into_iter().skip(1).fold(1, |acc, _| acc * 2)
}

#[cfg(test)]
mod tests {
    use crate::part_1;

    #[test]
    fn calculates_the_sum_for_part_1() {
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
}
