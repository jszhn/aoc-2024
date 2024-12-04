use itertools::enumerate;

advent_of_code::solution!(4);

fn search_part_one(input: &Vec<Vec<char>>, row: usize, col: usize) -> u32 {
    let directions = [
        (0, -1),
        (1, -1),
        (1, 0),
        (1, 1),
        (0, 1),
        (-1, 1),
        (-1, 0),
        (-1, -1),
    ];
    let letters = ['M', 'A', 'S'];

    let mut sum: u32 = 0;
    for dir in directions {
        let row_max = row as i32 + dir.1 * 3;
        let col_max = col as i32 + dir.0 * 3;

        if (row_max >= 0 && (row_max as usize) < input.len())
            && (col_max >= 0 && (col_max as usize) < input[0].len())
        {
            // bounds checking
            let mut flag = false;
            for i in 1..4 {
                let row_new = (row as i32 + dir.1 * i) as usize;
                let col_new = (col as i32 + dir.0 * i) as usize;

                if input[row_new][col_new] != letters[(i - 1) as usize] {
                    flag = true;
                    break;
                }
            }

            if !flag {
                sum += 1;
            }
        }
    }

    sum
}

pub fn part_one(input: &str) -> Option<u32> {
    let lines = input.lines();
    let vec: Vec<Vec<char>> = lines.map(|line| line.chars().collect()).collect();

    let mut sum: u32 = 0;
    for (r, row) in enumerate(&vec) {
        for (c, letter) in enumerate(row) {
            match letter {
                'X' => sum += search_part_one(&vec, r, c),
                _ => continue,
            }
        }
    }

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
        assert_eq!(result, Some(18));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
