use itertools::Itertools;

advent_of_code::solution!(2);

pub fn part_one(input: &str) -> Option<u32> {
    let mut safe: u32 = 0;
    let lines = input.lines();
    for line in lines {
        let nums: Vec<i32> = line
            .split_whitespace()
            .into_iter()
            .map(|x| x.parse().unwrap())
            .collect();
        let mut diff: Vec<i32> = Vec::with_capacity(nums.len() - 1);
        for (l, r) in nums.iter().tuple_windows() {
            diff.push(r - l);
        }

        // check number differences
        if diff
            .iter()
            .fold(0, |acc, x| acc + ((x.abs() > 3 || *x == 0) as i32))
            > 0
        {
            continue;
        }

        // check direction of monotonicity
        let pos = diff.iter().fold(0, |acc, x| acc + ((*x > 0) as i32));
        let neg = diff.iter().fold(0, |acc, x| acc + ((*x < 0) as i32));
        if (pos == diff.len() as i32 && neg == 0) || (neg == diff.len() as i32 && pos == 0) {
            safe += 1;
        }
    }
    Some(safe)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut safe: u32 = 0;
    let lines = input.lines();
    for line in lines {
        let nums: Vec<i32> = line
            .split_whitespace()
            .into_iter()
            .map(|x| x.parse().unwrap())
            .collect();
        let mut diff: Vec<i32> = Vec::with_capacity(nums.len() - 1);
        for (l, r) in nums.iter().tuple_windows() {
            diff.push(r - l);
        }

        let bad_diffs = diff
            .iter()
            .fold(0, |acc, x| acc + ((x.abs() > 3 || *x == 0) as i32));
        let pos = diff.iter().fold(0, |acc, x| acc + ((*x > 0) as i32));
        let neg = diff.iter().fold(0, |acc, x| acc + ((*x < 0) as i32));
        if bad_diffs == 0
            && ((pos == diff.len() as i32 && neg == 0)
            || (neg == diff.len() as i32 && pos == 0))
        {
            safe += 1;
            continue;
        }

        // remove one element and try again
        // not proud of this one
        for i in 0..nums.len() {
            let brute: Vec<i32> = [&nums[..i], &nums[i + 1..nums.len()]].concat();
            let mut diff: Vec<i32> = Vec::with_capacity(brute.len() - 1);
            for (l, r) in brute.iter().tuple_windows() {
                diff.push(r - l);
            }

            let bad_diffs = diff
                .iter()
                .fold(0, |acc, x| acc + ((x.abs() > 3 || *x == 0) as i32));
            let pos = diff.iter().fold(0, |acc, x| acc + ((*x > 0) as i32));
            let neg = diff.iter().fold(0, |acc, x| acc + ((*x < 0) as i32));
            if bad_diffs == 0
                && ((pos == diff.len() as i32 && neg == 0)
                || (neg == diff.len() as i32 && pos == 0))
            {
                safe += 1;
                break;
            }
        }
    }

    Some(safe)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4));
    }
}
