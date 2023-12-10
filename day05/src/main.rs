fn main() {
    let input = include_str!("input.txt");
    let res_part_1 = part_1(input);
    println!("{}", res_part_1);
    let res_part_2 = part_2(input);
    println!("{}", res_part_2);
}

struct RangeMapping {
    destination_start: i64,
    source_start: i64,
    range: i64,
}

impl RangeMapping {
    fn get_destination(&self, num: &i64) -> Option<i64> {
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

fn get_destinations(mappings: &Vec<RangeMapping>, num_ranges: Vec<Vec<i64>>) -> Vec<Vec<i64>> {
    let mut destinations: Vec<Vec<i64>> = Vec::new();
    let mut current_ranges: Vec<Vec<i64>> = Vec::from(num_ranges);

    loop {
        let mut remainders: Vec<Vec<i64>> = Vec::new();
        for r in current_ranges {
            let start = r[0];
            let range = r[1];
            let end = start + range - 1;
            let mut has_overlap: bool = false;

            for m in mappings {
                let m_end = m.source_start + m.range - 1;
                if is_overlapping(start, end, m.source_start, m_end) {
                    has_overlap = true;
                    let dest_start: i64;
                    let dest_end: i64;
                    let diff = m.destination_start - m.source_start;

                    if m.source_start <= start {
                        dest_start = start + diff;
                    } else {
                        dest_start = m.destination_start;
                        remainders.push(vec![start, m.source_start - 1 - start]);
                    }
                    if end <= m_end {
                        dest_end = end + diff;
                    } else {
                        dest_end = m.destination_start + m.range - 1;
                        remainders.push(vec![m_end + 1, end - m_end + 1]);
                    }

                    destinations.push(vec![dest_start, dest_end - dest_start + 1]);
                } else {
                    if has_overlap {
                        break;
                    }
                }
            }

            if !has_overlap && remainders.is_empty() {
                destinations.push(vec![start, range]);
            }
        }
        if remainders.is_empty() {
            break;
        }
        current_ranges = remainders;
    }
    destinations
}

fn is_overlapping(start_a: i64, end_a: i64, start_b: i64, end_b: i64) -> bool {
    start_a.max(start_b) <= end_a.min(end_b)
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
                        .find_map(|m| m.get_destination(num))
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
        .map(|block| {
            let mut mapping = block
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
                .collect::<Vec<RangeMapping>>();
            mapping.sort_by_key(|mapping| mapping.source_start);
            mapping
        })
        .collect();

    Almanac { seeds, maps }
}

fn part_2(input: &str) -> i64 {
    let almanac = parse_input(input);
    let seeds: Vec<Vec<i64>> = almanac
        .seeds
        .chunks(2)
        .map(|chunk| chunk.to_vec())
        .collect();

    let mut current_ranges = Vec::from(seeds);
    for mapping in almanac.maps {
        current_ranges = get_destinations(&mapping, current_ranges.clone());
    }
    current_ranges.iter().map(|v| v[0]).min().unwrap()
}

#[cfg(test)]
mod tests {
    use crate::{part_1, part_2};

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

    #[test]
    fn calculates_part_2_correctly() {
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
        let actual = part_2(input);
        let expected = 46;
        assert_eq!(actual, expected)
    }
}
