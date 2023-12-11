fn main() {
    let input = include_str!("input.txt");
    let res_part_1 = part_1(input);
    println!("{}", res_part_1);
    let res_part_2 = part_2(input);
    println!("{}", res_part_2);
}

fn part_1(input: &str) -> usize {
    let lines: Vec<_> = input
        .lines()
        .map(|line| {
            line.get(10..)
                .unwrap()
                .trim()
                .split_whitespace()
                .filter_map(|n| n.trim().parse::<i32>().ok())
                .collect::<Vec<_>>()
        })
        .collect();

    let times = &lines[0];
    let records = &lines[1];

    times
        .into_iter()
        .enumerate()
        .map(|(i, time)| {
            let mut distances: Vec<i32> = Vec::new();

            for speed in 0..time + 1 {
                let race = time - speed;
                distances.push(race * speed);
            }
            let record = &records[i];
            distances.into_iter().filter(|d| d > record).count()
        })
        .fold(1, |acc, n| acc * n)
}

fn part_2(input: &str) -> usize {
    let lines: Vec<_> = input
        .lines()
        .map(|line| {
            line.get(10..)
                .unwrap()
                .trim()
                .replace(" ", "")
                .parse::<usize>()
                .unwrap()
        })
        .collect();

    let time = &lines[0];
    let record = &lines[1];

    let mut distances: Vec<usize> = Vec::new();
    for speed in 0..time / 2 + 1 {
        let race = time - speed;
        distances.push(race * speed);
    }
    let res = distances.into_iter().filter(|d| d > record).count() * 2;

    if time % 2 == 1 {
        return res;
    } else {
        return res - 1;
    }
}

#[cfg(test)]
mod tests {
    use crate::{part_1, part_2};

    #[test]
    fn calculates_part_1_correctly() {
        let input = "Time:      7  15   30
Distance:  9  40  200";
        let actual = part_1(input);
        let expected = 288;
        assert_eq!(actual, expected)
    }

    #[test]
    fn calculates_part_2_correctly() {
        let input = "Time:      7  15   30
Distance:  9  40  200";
        let actual = part_2(input);
        let expected = 71503;
        assert_eq!(actual, expected)
    }
}
