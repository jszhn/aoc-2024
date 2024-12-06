use std::collections::{HashMap, HashSet};

use itertools::{enumerate, Itertools};

advent_of_code::solution!(5);

fn parse(input: &str) -> (HashMap<u32, Vec<u32>>, Vec<Vec<u32>>) {
    let mut lines = input.lines();
    let dependencies: Vec<(u32, u32)> = lines
        .by_ref()
        .take_while(|x| !x.is_empty())
        .map(|x| x.split('|').collect_tuple().unwrap()) // split to str
        .map(|(before, after): (&str, &str)| (before.parse().unwrap(), after.parse().unwrap())) // convert to u32
        .collect();
    let updates: Vec<Vec<u32>> = lines
        .map(|x| {
            x.split(',')
                .collect::<Vec<&str>>()
                .iter()
                .map(|x| x.parse().unwrap())
                .collect()
        })
        .collect();

    // construct graph adjacency list
    let mut adj: HashMap<u32, Vec<u32>> = HashMap::new();
    for (before, after) in &dependencies {
        adj.entry(*before).or_default().push(*after);
    }

    (adj, updates)
}

fn validate_topological_order(sequence: &[u32], adj: &HashMap<u32, Vec<u32>>) -> bool {
    // edge approach for validating topological order, O(VE)
    // basically creates a reverse mapping and does a look-up
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
            }
        }
    }

    true
}

pub fn part_one(input: &str) -> Option<u32> {
    let (adj, updates) = parse(input);
    let correct: Vec<&Vec<u32>> = updates
        .iter()
        .filter(|u| validate_topological_order(u, &adj))
        .collect();
    let sum: u32 = correct.iter().map(|seq| seq[seq.len() / 2]).sum();
    Some(sum)
}

fn dfs(v: u32, adj: &HashMap<u32, Vec<u32>>, visited: &mut HashSet<u32>, stack: &mut Vec<u32>, candidates: &HashSet<&u32>) {
    visited.insert(v);

    if let Some(neighbours) = adj.get(&v) {
        for n in neighbours {
            if candidates.contains(n) && !visited.contains(n) {
                dfs(*n, adj, visited, stack, candidates);
            }
        }
    }

    stack.push(v);
}

fn topological_sort(adj: &HashMap<u32, Vec<u32>>, sequence: &[u32]) -> Vec<u32> {
    let candidates: HashSet<&u32> = HashSet::from_iter(sequence.iter());
    let mut stack: Vec<u32> = Vec::with_capacity(adj.len());
    let mut visited: HashSet<u32> = HashSet::with_capacity(adj.len());

    for num in sequence {
        if !visited.contains(num) {
            dfs(*num, adj, &mut visited, &mut stack, &candidates);
        }
    }

    stack.reverse();
    stack
}

pub fn part_two(input: &str) -> Option<u32> {
    let (adj, updates) = parse(input);
    let incorrect: Vec<&Vec<u32>> = updates
        .iter()
        .filter(|u| !validate_topological_order(u, &adj))
        .collect();
    let sum: u32 = incorrect.iter().map(|u| {
        let sorted = topological_sort(&adj, u);
        sorted[sorted.len() / 2]
    }).sum();
    Some(sum)
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
        assert_eq!(result, Some(123));
    }
}
