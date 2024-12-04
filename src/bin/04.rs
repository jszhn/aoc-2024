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

    let sum = directions
        .iter()
        .filter(|dir| {
            let row_max = row as i32 + dir.1 * 3;
            let col_max = col as i32 + dir.0 * 3;

            row_max >= 0
                && (row_max as usize) < input.len()
                && (col_max >= 0 && (col_max as usize) < input[0].len())
        })
        .filter(|dir| {
            (1..4)
                .map(|i| {
                    let row_new = (row as i32 + dir.1 * i) as usize;
                    let col_new = (col as i32 + dir.0 * i) as usize;
                    input[row_new][col_new] == letters[(i - 1) as usize]
                })
                .all(|x| x)
        })
        .count() as u32;
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

fn search_part_two(input: &Vec<Vec<char>>, row: usize, col: usize) -> u32 {
    let mut sum: u32 = 0;
    if row <= input.len() - 3 {
        // try left
        if col >= 2
            && [
                input[row + 1][col - 1] == 'A',
                input[row + 2][col] == 'M',
                input[row][col - 2] == 'S',
                input[row + 2][col - 2] == 'S',
            ]
            .iter()
            .map(|c| *c as u8)
            .sum::<u8>()
                == 4
        {
            sum += 1;
        }

        // try right
        if col <= input[0].len() - 3
            && [
                input[row + 1][col + 1] == 'A',
                input[row + 2][col] == 'M',
                input[row][col + 2] == 'S',
                input[row + 2][col + 2] == 'S',
            ]
            .iter()
            .map(|c| *c as u8)
            .sum::<u8>()
                == 4
        {
            sum += 1;
        }

        // try down
        if col <= input[0].len() - 3
            && [
                input[row + 1][col + 1] == 'A',
                input[row][col + 2] == 'M',
                input[row + 2][col] == 'S',
                input[row + 2][col + 2] == 'S',
            ]
            .iter()
            .map(|c| *c as u8)
            .sum::<u8>()
                == 4
        {
            sum += 1;
        }
    }

    if row >= 2
        && col <= input[0].len() - 3
        && [
            // try up
            input[row - 1][col + 1] == 'A',
            input[row][col + 2] == 'M',
            input[row - 2][col] == 'S',
            input[row - 2][col + 2] == 'S',
        ]
        .iter()
        .map(|c| *c as u8)
        .sum::<u8>()
            == 4
    {
        sum += 1;
    }

    sum
}

pub fn part_two(input: &str) -> Option<u32> {
    let lines = input.lines();
    let vec: Vec<Vec<char>> = lines.map(|line| line.chars().collect()).collect();

    let mut sum: u32 = 0;
    for (r, row) in enumerate(&vec) {
        for (c, letter) in enumerate(row) {
            match letter {
                'M' => sum += search_part_two(&vec, r, c),
                _ => continue,
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
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(18));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(9));
    }
}
