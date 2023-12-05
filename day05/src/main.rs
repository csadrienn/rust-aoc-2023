use std::fmt::Alignment;

fn main() {
    let input = include_str!("input.txt");
    let res_part_1 = part_1(input);
    println!("{}", res_part_1);
}

struct RangeMapping {
    destination_start: i64,
    source_start: i64,
    range: i64,
}

impl RangeMapping {
    fn get_source(&self, num: &i64) -> Option<i64> {
        if let Some(diff) =
            (num.checked_sub(self.source_start)).filter(|&n| n >= 0 && n < self.range)
        {
            Some(self.destination_start + diff)
        } else {
            None
        }
    }
}

struct Almanac {
    seeds: Vec<i64>,
    maps: Vec<Vec<RangeMapping>>,
}

fn part_1(input: &str) -> i64 {
    let almanac = parse_input(input);
    almanac
        .maps
        .into_iter()
        .fold(almanac.seeds, |acc, mappings| {
            acc.iter()
                .map(|num| {
                    mappings
                        .iter()
                        .find_map(|m| m.get_source(num))
                        .unwrap_or(*num)
                })
                .collect()
        })
        .into_iter()
        .min()
        .unwrap()
}

fn parse_input(input: &str) -> Almanac {
    let mut blocks = input.split("\n\n");
    let seeds: Vec<i64> = blocks
        .next()
        .unwrap()
        .split_whitespace()
        .skip(1)
        .map(|n| n.trim().parse().unwrap())
        .collect();

    let maps = blocks
        .skip(1)
        .map(|block| {
            block
                .lines()
                .skip(1)
                .map(|mapping| {
                    let mut nums = mapping
                        .split_whitespace()
                        .map(|n| n.parse::<i64>().unwrap());
                    RangeMapping {
                        destination_start: nums.next().unwrap(),
                        source_start: nums.next().unwrap(),
                        range: nums.next().unwrap(),
                    }
                })
                .collect::<Vec<RangeMapping>>()
        })
        .collect();
    Almanac { seeds, maps }
}

#[cfg(test)]
mod tests {
    use crate::part_1;

    #[test]
    fn calculates_part_1_correctly() {
        let input = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";
        let actual = part_1(input);
        let expected = 35;
        assert_eq!(actual, expected)
    }
}
