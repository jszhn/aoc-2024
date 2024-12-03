use itertools::Itertools;
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

pub fn part_two(input: &str) -> Option<u32> {
    let re = Regex::new(r"(mul\([0-9]+,[0-9]+\))|((do|don't)\(\))").unwrap();
    let nums = Regex::new(r"(?<f>[0-9]+),(?<s>[0-9]+)").unwrap();
    let instructions: Vec<&str> = re.find_iter(input).map(|str| str.as_str()).collect();

    let mut sum: u32 = 0;
    let mut toggle: bool = true;
    for instr in instructions {
        match instr {
            "do()" => toggle = true,
            "don't()" => toggle = false,
            _ => {
                if !toggle {
                    continue;
                } else {
                    let captures = nums.captures(instr).unwrap();
                    let first: u32 = captures.name("f").unwrap().as_str().parse().unwrap();
                    let second: u32 = captures.name("s").unwrap().as_str().parse().unwrap();

                    sum += first * second;
                }
            }
        }
    }

    Some(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file_part("examples", DAY, 1));
        assert_eq!(result, Some(161));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part("examples", DAY, 2));
        assert_eq!(result, Some(48));
    }
}
