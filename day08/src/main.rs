use num::Integer;
use std::collections::HashMap;

fn main() {
    let input = include_str!("input.txt");
    let res_part_1 = part_1(input);
    println!("{}", res_part_1);
    let res_part_2 = part_2(input);
    println!("{}", res_part_2);
}

fn part_1(input: &str) -> usize {
    let (directions, node_map, _) = parse_input(input);
    get_steps(&directions, &node_map, "AAA", "ZZZ")
}

fn part_2(input: &str) -> usize {
    let (directions, node_map, starting_nodes) = parse_input(input);
    starting_nodes
        .into_iter()
        .map(|n| get_steps(&directions, &node_map, n, "Z"))
        .fold(1, |a, b: usize| a.lcm(&b))
}

fn parse_input(input: &str) -> (Vec<char>, HashMap<&str, (&str, &str)>, Vec<&str>) {
    let (directions_str, nodes_str) = input.split_once("\n\n").unwrap();
    let directions: Vec<char> = directions_str.chars().collect();
    let mut starting_nodes: Vec<&str> = Vec::new();
    let mut node_map = HashMap::new();
    nodes_str.lines().for_each(|line| {
        let (node, next_str) = line.split_once(" = ").unwrap();
        let next_nodes = next_str
            .get(1..next_str.len() - 1)
            .unwrap()
            .split_once(", ")
            .unwrap();
        node_map.insert(node, next_nodes);
        if node.ends_with("A") {
            starting_nodes.push(node);
        }
    });

    (directions, node_map, starting_nodes)
}

fn get_steps(
    directions: &Vec<char>,
    node_map: &HashMap<&str, (&str, &str)>,
    start_node: &str,
    end_node: &str,
) -> usize {
    let mut dir_index = 0;
    let mut node = start_node;
    let mut steps = 0;

    while !node.ends_with(end_node) {
        let next_node_options = node_map.get(node).unwrap();
        let dir = directions[dir_index];
        node = match dir {
            'L' => next_node_options.0,
            _ => next_node_options.1,
        };
        dir_index = (dir_index + 1) % directions.len();
        steps += 1;
    }
    steps
}

#[cfg(test)]
mod tests {
    use crate::{part_1, part_2};

    #[test]
    fn calculates_part_1_correctly() {
        let input1 = "RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)";
        let actual1 = part_1(input1);
        let expected1 = 2;
        assert_eq!(actual1, expected1, "on a single section");
        let input1 = "LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)";
        let actual2 = part_1(input1);
        let expected2 = 6;
        assert_eq!(actual2, expected2, "with repeated sections");
    }

    #[test]
    fn calculates_part_2_correctly() {
        let input = "LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)";
        let actual = part_2(input);
        let expected = 6;
        assert_eq!(actual, expected)
    }
}
