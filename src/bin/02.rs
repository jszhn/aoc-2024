advent_of_code::solution!(2);

pub fn part_one(input: &str) -> Option<u32> {
    let mut safe: u32 = 0;
    let lines = input.lines();
    for line in lines {
        let mut nums: Vec<i32> = line
            .split_whitespace()
            .into_iter()
            .map(|x| x.parse().unwrap())
            .collect();

        // check number differences
        let mut cert = true;
        let mut prev = &nums[0] + 1;
        for num in &nums {
            if (num - prev).abs() > 3 || (num - prev.abs()) == 0 {
                cert = false;
                break;
            } else {
                prev = *num;
            }
        }
        if !cert {
            continue;
        }

        // check direction of monotonicity
        let (mut dec, mut inc) = (false, false);
        prev = nums.pop().unwrap();
        for num in nums.iter().rev() {
            inc |= prev > *num;
            dec |= prev < *num;
            prev = *num;
        }
        if dec != inc {
            safe += 1;
        }
    }

    Some(safe)
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
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
