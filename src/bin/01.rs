use std::cmp::{max, min};
use std::collections::HashMap;

advent_of_code::solution!(1);

fn parse(input: &str) -> (Vec<u32>, Vec<u32>) {
    let mut left: Vec<u32> = Vec::with_capacity(1000);
    let mut right: Vec<u32> = Vec::with_capacity(1000);

    let lines = input.lines();
    for line in lines {
        let mut vals = line.split_whitespace();
        left.push(vals.next().unwrap().parse().unwrap());
        right.push(vals.next().unwrap().parse().unwrap());
    }

    (left, right)
}

pub fn part_one(input: &str) -> Option<u32> {
    let (mut left, mut right) = parse(input);
    left.sort();
    right.sort();

    let distance: u32 = left
        .iter()
        .zip(right.iter())
        .map(|(l, r)| max(l, r) - min(l, r))
        .sum();
    Some(distance)
}

pub fn part_two(input: &str) -> Option<u32> {
    let (left, right) = parse(input);
    let mut hash: HashMap<u32, u32> = HashMap::new();
    for r in right {
        if let Some(elem) = hash.get_mut(&r) {
            *elem += 1;
        } else {
            hash.insert(r, 1);
        }
    }

    let similarity: u32 = left.iter().map(|l| hash.get(l).unwrap_or(&0) * l).sum();
    Some(similarity)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(31));
    }
}
