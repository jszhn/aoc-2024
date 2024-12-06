use std::collections::HashMap;

use itertools::{enumerate, Itertools};

advent_of_code::solution!(5);

fn parse(input: &str) -> (Vec<(u32, u32)>, Vec<Vec<u32>>) {
    let mut lines = input.lines();
    let dependencies: Vec<(u32, u32)> = lines
        .by_ref()
        .take_while(|x| !x.is_empty())
        .map(|x| x.split("|").collect_tuple().unwrap()) // split to str
        .map(|(before, after): (&str, &str)| (before.parse().unwrap(), after.parse().unwrap())) // convert to u32
        .collect();
    let updates: Vec<Vec<u32>> = lines
        .map(|x| {
            x.split(",")
                .collect::<Vec<&str>>()
                .iter()
                .map(|x| x.parse().unwrap())
                .collect()
        })
        .collect();

    (dependencies, updates)
}

fn validate(sequence: &Vec<u32>, adj: &HashMap<u32, Vec<u32>>) -> bool {
    // edge approach for validating topological order, O(VE)
    let mut node_to_index: HashMap<u32, usize> = HashMap::new();
    for (it, val) in enumerate(sequence.iter()) {
        node_to_index.entry(*val).or_insert(it);
    }

    for (u, neighbours) in adj {
        for v in neighbours {
            if let (Some(&ux), Some(&vx)) = (node_to_index.get(u), node_to_index.get(v)) {
                if ux >= vx {
                    return false;
                }
            } else {
                continue;
            }
        }
    }

    true
}

pub fn part_one(input: &str) -> Option<u32> {
    let (dependencies, updates) = parse(input);

    // construct graph adjacency list
    let mut adj: HashMap<u32, Vec<u32>> = HashMap::new();
    for (before, after) in dependencies {
        adj.entry(before).or_default().push(after);
    }

    let correct: Vec<&Vec<u32>> = updates.iter().filter(|u| validate(u, &adj)).collect();
    let sum: u32 = correct.iter().map(|seq| seq[seq.len() / 2]).sum();
    Some(sum)
}

pub fn part_two(_input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(143));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
