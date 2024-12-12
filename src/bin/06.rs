advent_of_code::solution!(6);

#[derive(Clone, Copy)]
enum Direction {
    Up = 0,
    Right,
    Down,
    Left,
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    let moves = [(-1, 0), (0, 1), (1, 0), (0, -1)];
    let mut dir: Direction = Direction::Up;
    let mut pos: (i32, i32) = (0, 0);
    'outer: for (row, vec) in grid.iter().enumerate() {
        for (col, c) in vec.iter().enumerate() {
            if *c != '.' && *c != '#' {
                match *c {
                    '^' => dir = Direction::Up,
                    '>' => dir = Direction::Right,
                    'v' => dir = Direction::Down,
                    '<' => dir = Direction::Left,
                    _ => unreachable!(),
                }
                pos = (row as i32, col as i32);
                break 'outer;
            }
        }
    }

    loop {
        let (row, col) = pos;
        if row < 0 || row >= grid.len() as i32 || col < 0 || col >= grid[0].len() as i32 {
            break;
        }

        let elem = grid[row as usize][col as usize];
        if elem == '#' {
            pos.0 = row - moves[dir as usize].0;
            pos.1 = col - moves[dir as usize].1;
            dir = match dir {
                Direction::Up => Direction::Right,
                Direction::Right => Direction::Down,
                Direction::Down => Direction::Left,
                Direction::Left => Direction::Up,
            };
        } else {
            grid[row as usize][col as usize] = 'X';
            pos.0 = row + moves[dir as usize].0;
            pos.1 = col + moves[dir as usize].1;
        }
    }

    let count: u32 = grid
        .iter()
        .flat_map(|row| row.iter().filter(|&&elem| elem == 'X'))
        .count() as u32;
    Some(count)
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
        assert_eq!(result, Some(41));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
