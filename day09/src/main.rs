fn main() {
    let input = include_str!("input.txt");
    println!("{:?}", part_1(input));
    println!("{:?}", part_2(input));
}

fn part_1(input: &str) -> i32 {
    input
        .lines()
        .map(|line| {
            let numbers: Vec<i32> = line
                .split_whitespace()
                .map(|n| n.parse().unwrap())
                .collect();

            let mut history_endings: Vec<i32> = get_history_sequences(numbers)
                .into_iter()
                .map(|s| s[s.len() - 1])
                .collect();

            history_endings.reverse();

            let mut ending = 0;
            history_endings.iter().skip(1).for_each(|n| {
                ending = n + ending;
            });
            ending
        })
        .sum()
}
fn part_2(input: &str) -> i32 {
    input
        .lines()
        .map(|line| {
            let numbers: Vec<i32> = line
                .split_whitespace()
                .map(|n| n.parse().unwrap())
                .collect();

            let mut history_starts: Vec<i32> = get_history_sequences(numbers)
                .into_iter()
                .map(|s| s[0])
                .collect();
            history_starts.reverse();

            let mut start = 0;
            history_starts.iter().skip(1).for_each(|n| {
                start = n - start;
            });
            start
        })
        .sum()
}

fn get_history_sequences(numbers: Vec<i32>) -> Vec<Vec<i32>> {
    let mut history_sequences = Vec::new();
    history_sequences.push(numbers.clone());

    if numbers.iter().all(|n| *n == 0) {
        return history_sequences;
    }
    let next_numbers = numbers
        .iter()
        .enumerate()
        .skip(1)
        .map(|(i, n)| n - numbers[i - 1])
        .collect();
    history_sequences.append(&mut get_history_sequences(next_numbers));
    history_sequences
}

#[cfg(test)]
mod tests {
    use crate::{part_1, part_2};

    #[test]
    fn calculates_part_1_correctly() {
        let input = "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45";
        let actual = part_1(input);
        let expected = 114;
        assert_eq!(actual, expected);
    }

    #[test]
    fn calculates_part_2_correctly() {
        let input = "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45";
        let actual = part_2(input);
        let expected = 2;
        assert_eq!(actual, expected);
    }
}
