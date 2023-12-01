fn main() {
    let input = include_str!("input.txt");
    let part_1 = day01_part_1(input);
    println!("{}", part_1);
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

#[cfg(test)]
mod tests {
    use crate::day01_part_1;
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
}
