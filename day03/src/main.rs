fn main() {
    let input = include_str!("input.txt");
    let sum_part_1 = part_1(input);
    let sum_part_2 = part_2(input);
    println!("{}", sum_part_1);
    println!("{}", sum_part_2);
}

struct Number {
    value: i32,
    line: usize,
    start_index: usize,
    end_index: usize,
}

fn part_1(input: &str) -> i32 {
    let mut symbols: Vec<(usize, usize)> = vec![];
    let mut numbers: Vec<Number> = vec![];

    input.lines().enumerate().for_each(|(line_index, line)| {
        let mut current_num = String::new();
        line.char_indices().for_each(|(char_index, c)| {
            if c.is_numeric() {
                current_num.push(c);
                if char_index == line.len() - 1 {
                    numbers.push(Number {
                        value: current_num.parse().unwrap(),
                        line: line_index,
                        start_index: char_index - current_num.len(),
                        end_index: char_index - 1,
                    });
                    current_num = String::new();
                }
            } else {
                if c != '.' {
                    symbols.push((line_index, char_index))
                }
                if !current_num.is_empty() {
                    numbers.push(Number {
                        value: current_num.parse().unwrap(),
                        line: line_index,
                        start_index: char_index - current_num.len(),
                        end_index: char_index - 1,
                    });
                    current_num = String::new();
                }
            }
        });
    });
    numbers
        .into_iter()
        .filter_map(|n| is_part_number(&n, &symbols).then(|| n.value))
        .sum()
}

fn is_part_number(number: &Number, symbols: &Vec<(usize, usize)>) -> bool {
    let found_symbols = symbols.iter().find(|s| {
        let line_diff = number.line.abs_diff(s.0);
        let symbol_in_range =
            (number.start_index..number.end_index + 1).find(|i| i.abs_diff(s.1) <= 1);

        line_diff <= 1 && symbol_in_range.is_some()
    });
    found_symbols.is_some()
}

fn part_2(input: &str) -> i32 {
    let mut star_symbols: Vec<(usize, usize)> = vec![];
    let mut numbers: Vec<Number> = vec![];

    input.lines().enumerate().for_each(|(line_index, line)| {
        let mut current_num = String::new();
        line.char_indices().for_each(|(char_index, c)| {
            if c.is_numeric() {
                current_num.push(c);
                if char_index == line.len() - 1 {
                    numbers.push(Number {
                        value: current_num.parse().unwrap(),
                        line: line_index,
                        start_index: char_index - current_num.len(),
                        end_index: char_index - 1,
                    });
                    current_num = String::new();
                }
            } else {
                if c == '*' {
                    star_symbols.push((line_index, char_index))
                }
                if !current_num.is_empty() {
                    numbers.push(Number {
                        value: current_num.parse().unwrap(),
                        line: line_index,
                        start_index: char_index - current_num.len(),
                        end_index: char_index - 1,
                    });
                    current_num = String::new();
                }
            }
        });
    });
    star_symbols
        .into_iter()
        .filter_map(|s| get_gear_ratio(s, &numbers))
        .sum()
}

fn get_gear_ratio(symbol: (usize, usize), numbers: &Vec<Number>) -> Option<i32> {
    let adjacent_nums: Vec<_> = numbers
        .into_iter()
        .filter(|number| {
            let line_diff = number.line.abs_diff(symbol.0);
            let symbol_in_range =
                (number.start_index..number.end_index + 1).find(|i| i.abs_diff(symbol.1) <= 1);
            line_diff <= 1 && symbol_in_range.is_some()
        })
        .collect();
    if adjacent_nums.len() == 2 {
        Some(adjacent_nums[0].value * adjacent_nums[1].value)
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use crate::{part_1, part_2};

    #[test]
    fn calculates_the_sum_for_part_1() {
        let input = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";
        let actual = part_1(input);
        let expected = 4361;
        assert_eq!(actual, expected)
    }

    #[test]
    fn calculates_the_sum_for_part_2() {
        let input = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";
        let actual = part_2(input);
        let expected = 467835;
        assert_eq!(actual, expected)
    }
}
