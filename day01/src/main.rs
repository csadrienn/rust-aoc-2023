fn main() {
    let input = include_str!("input.txt");
    let part_1 = day01_part_1(input);
    let part_2 = day01_part_2(input);
    println!("{}", part_1);
    println!("{}", part_2);
}

fn day01_part_1(input: &str) -> u32 {
    input
        .lines()
        .map(|line| {
            let chars: Vec<char> = line.chars().filter(|c| c.is_digit(10)).collect();
            format!("{}{}", chars.first().unwrap(), chars.last().unwrap())
                .parse::<u32>()
                .unwrap()
        })
        .sum()
}

fn parse_spelled_nums<'a>(numbers: &'a Vec<[&'a str; 2]>, num_str: &'a str) -> &'a str {
    for pair in numbers {
        if pair[0] == num_str {
            return pair[1].clone();
        }
    }
    num_str.clone()
}

fn day01_part_2(input: &str) -> u32 {
    let numbers: Vec<[&str; 2]> = vec![
        ["one", "1"],
        ["two", "2"],
        ["three", "3"],
        ["four", "4"],
        ["five", "5"],
        ["six", "6"],
        ["seven", "7"],
        ["eight", "8"],
        ["nine", "9"],
    ];
    input
        .lines()
        .map(|line| {
            let mut first_nums: Vec<_> = numbers
                .iter()
                .flatten()
                .map(|n| {
                    let index = line.find(n);
                    (n, index)
                })
                .filter(|num| num.1.is_some())
                .collect();

            first_nums.sort_by(|a, b| a.1.unwrap().cmp(&b.1.unwrap()));

            let mut last_nums: Vec<_> = numbers
                .iter()
                .flatten()
                .map(|n| {
                    let index = line.rfind(n);
                    (n, index)
                })
                .filter(|num| num.1.is_some())
                .collect();
            last_nums.sort_by(|a, b| a.1.unwrap().cmp(&b.1.unwrap()));

            let first = first_nums.first().unwrap();
            let last = last_nums.last().unwrap();

            format!(
                "{}{}",
                parse_spelled_nums(&numbers, first.0),
                parse_spelled_nums(&numbers, last.0)
            )
            .parse::<u32>()
            .unwrap()
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use crate::{day01_part_1, day01_part_2};
    #[test]
    fn calculates_the_sum_for_part_1() {
        let input = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";
        let actual = day01_part_1(input);
        let expected = 142;
        assert_eq!(actual, expected)
    }

    #[test]
    fn calculates_the_sum_for_part_2() {
        let input = "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen";
        let actual = day01_part_2(input);
        let expected = 281;
        assert_eq!(actual, expected)
    }
}
