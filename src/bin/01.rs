use std::cmp::{max, min};

advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u32> {
    let mut left: Vec<u32> = Vec::with_capacity(1000);
    let mut right: Vec<u32> = Vec::with_capacity(1000);

    let lines = input.lines();
    for line in lines {
        let mut vals = line.split_whitespace();
        left.push(vals.next().unwrap().parse().unwrap());
        right.push(vals.next().unwrap().parse().unwrap());
    }

    left.sort();
    right.sort();

    let distance: u32 = left.iter().zip(right.iter())
        .map(|(l, r)| max(l, r) - min(l, r))
        .sum();
    Some(distance)
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
        assert_eq!(result, Some(11));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
