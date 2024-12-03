use regex::Regex;

advent_of_code::solution!(3);

pub fn part_one(input: &str) -> Option<u32> {
    // fastest time probably defining DFA and searching in O(n)
    // I don't do that though
    let re = Regex::new(r"mul\((?<f>[0-9]+),(?<s>[0-9]+)\)").unwrap();
    let instructions: Vec<(u32, u32)> = re
        .captures_iter(input)
        .map(|str| {
            let first: u32 = str.name("f").unwrap().as_str().parse().unwrap();
            let second: u32 = str.name("s").unwrap().as_str().parse().unwrap();
            (first, second)
        })
        .collect();

    let sum: u32 = instructions.iter().map(|tup| tup.0 * tup.1).sum();
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
        assert_eq!(result, Some(161));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
