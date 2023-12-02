fn main() {
    let input = include_str!("input.txt");
    let sum_part_1 = part_1(input);
    let sum_part_2 = part_2(input);
    println!("{}", sum_part_1);
    println!("{}", sum_part_2);
}

fn part_1(input: &str) -> u32 {
    input
        .lines()
        .map(|line| {
            let (game, sets) = line.split_once(':').unwrap();
            let game_id: u32 = game.get(5..game.len()).unwrap().parse().unwrap();
            sets.split(';')
                .all(|set| check_if_possible_set(set))
                .then(|| game_id)
                .unwrap_or(0)
        })
        .sum()
}

fn check_if_possible_set(set: &str) -> bool {
    set.split(',').all(|block| {
        let (num, color) = block.trim().split_once(' ').unwrap();
        let parsed_num = num.parse::<u32>().unwrap();
        match color {
            "red" => parsed_num <= 12,
            "green" => parsed_num <= 13,
            "blue" => parsed_num <= 14,
            _ => false,
        }
    })
}

fn part_2(input: &str) -> u32 {
    input
        .lines()
        .map(|line| {
            let (_, sets) = line.split_once(':').unwrap();
            get_multiple_of_max_values(sets)
        })
        .sum()
}

fn get_multiple_of_max_values(sets: &str) -> u32 {
    let mut red: u32 = 1;
    let mut green: u32 = 1;
    let mut blue: u32 = 1;

    sets.split(&[';', ',']).for_each(|cubes| {
        let (num, colour) = cubes.trim().split_once(' ').unwrap();
        let parsed_num = num.parse::<u32>().unwrap();
        match colour {
            "red" => red = red.max(parsed_num),
            "green" => green = green.max(parsed_num),
            "blue" => blue = blue.max(parsed_num),
            _ => (),
        }
    });
    red * green * blue
}

#[cfg(test)]
mod tests {
    use crate::{part_1, part_2};

    #[test]
    fn calculates_the_sum_for_part_1() {
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
        let actual = part_1(input);
        let expected = 8;
        assert_eq!(actual, expected)
    }

    #[test]
    fn calculates_the_sum_for_part_2() {
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
        let actual = part_2(input);
        let expected = 2286;
        assert_eq!(actual, expected)
    }
}
