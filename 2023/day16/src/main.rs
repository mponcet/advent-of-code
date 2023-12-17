use rayon::prelude::*;
use std::collections::HashMap;

#[derive(Debug)]
struct Grid {
    grid: Vec<u8>,
    columns: usize,
    rows: usize,
}

impl Grid {
    fn get(&self, i: usize, j: usize) -> Option<u8> {
        if i < self.columns && j < self.rows {
            self.grid.get(j * self.columns + i).copied()
        } else {
            None
        }
    }
}

fn parse(input: &str) -> Grid {
    let mut columns = 0;
    let grid: Vec<_> = input
        .lines()
        .flat_map(|l| {
            columns = l.len();
            l.as_bytes()
        })
        .copied()
        .collect();

    let rows = grid.len() / columns;

    Grid {
        grid,
        columns,
        rows,
    }
}

// Up => j + 1, Down => j - 1
#[derive(Debug, Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

fn dfs(
    grid: &Grid,
    visited: &mut HashMap<(usize, usize), usize>,
    i: isize,
    j: isize,
    direction: Direction,
    count: usize,
) {
    // if i >= grid.columns as isize || j >= grid.rows as isize || i < 0 || j < 0 {
    //     return;
    // }
    //
    if count > 800 {
        return;
    }

    if let Some(c) = grid.get(i as usize, j as usize) {
        let visit_count = visited
            .entry((i as usize, j as usize))
            .and_modify(|count| *count += 1)
            .or_insert(1);
        if *visit_count > 10000 {
            return;
        }

        match direction {
            Direction::Up => match c {
                b'.' | b'|' => dfs(grid, visited, i, j + 1, Direction::Up, count + 1),
                b'-' => {
                    dfs(grid, visited, i + 1, j, Direction::Right, count + 1);
                    dfs(grid, visited, i - 1, j, Direction::Left, count + 1);
                }
                b'\\' => dfs(grid, visited, i + 1, j, Direction::Right, count + 1),
                b'/' => dfs(grid, visited, i - 1, j, Direction::Left, count + 1),
                _ => unreachable!(),
            },
            Direction::Down => match c {
                b'.' | b'|' => dfs(grid, visited, i, j - 1, Direction::Down, count + 1),
                b'-' => {
                    dfs(grid, visited, i + 1, j, Direction::Right, count + 1);
                    dfs(grid, visited, i - 1, j, Direction::Left, count + 1);
                }
                b'\\' => dfs(grid, visited, i - 1, j, Direction::Left, count + 1),
                b'/' => dfs(grid, visited, i + 1, j, Direction::Right, count + 1),
                _ => unreachable!(),
            },
            Direction::Left => match c {
                b'.' | b'-' => dfs(grid, visited, i - 1, j, Direction::Left, count + 1),
                b'|' => {
                    dfs(grid, visited, i, j + 1, Direction::Up, count + 1);
                    dfs(grid, visited, i, j - 1, Direction::Down, count + 1);
                }
                b'\\' => dfs(grid, visited, i, j - 1, Direction::Down, count + 1),
                b'/' => dfs(grid, visited, i, j + 1, Direction::Up, count + 1),
                _ => unreachable!(),
            },
            Direction::Right => match c {
                b'.' | b'-' => dfs(grid, visited, i + 1, j, Direction::Right, count + 1),
                b'|' => {
                    dfs(grid, visited, i, j + 1, Direction::Up, count + 1);
                    dfs(grid, visited, i, j - 1, Direction::Down, count + 1);
                }
                b'\\' => dfs(grid, visited, i, j + 1, Direction::Up, count + 1),
                b'/' => dfs(grid, visited, i, j - 1, Direction::Down, count + 1),
                _ => unreachable!(),
            },
        }
    }
}

fn part1(input: &str) -> usize {
    let grid = parse(input);
    let mut visited = HashMap::new();

    dfs(&grid, &mut visited, 0, 0, Direction::Right, 0);
    visited.len()
}

fn part2(input: &str) -> usize {
    let grid = parse(input);
    let mut max_a = 0;
    let mut max_b = 0;

    for (j, direction) in [
        (0, Direction::Right),
        (0, Direction::Left),
        (0, Direction::Up),
        (grid.rows - 1, Direction::Right),
        (grid.rows - 1, Direction::Left),
        (grid.rows - 1, Direction::Down),
    ]
    .into_iter()
    {
        let max = (0..grid.columns)
            .into_par_iter()
            .map(|i| {
                let mut visited = HashMap::new();
                dfs(&grid, &mut visited, i as isize, j as isize, direction, 0);
                visited.len()
            })
            .max()
            .unwrap();

        if max > max_a {
            max_a = max;
        }
    }
    for (i, direction) in [
        (0, Direction::Right),
        (0, Direction::Down),
        (0, Direction::Up),
        (grid.columns - 1, Direction::Left),
        (grid.columns - 1, Direction::Down),
        (grid.columns - 1, Direction::Up),
    ]
    .into_iter()
    {
        let max = (0..grid.rows)
            .into_par_iter()
            .map(|j| {
                let mut visited = HashMap::new();
                dfs(&grid, &mut visited, i as isize, j as isize, direction, 0);
                visited.len()
            })
            .max()
            .unwrap();

        if max > max_b {
            max_b = max;
        }
    }

    max_a.max(max_b)
}

fn main() {
    println!("part1={}", part1(include_str!("../input.txt")));
    println!("part2={}", part2(include_str!("../input.txt")));
}

#[cfg(test)]
mod tests {
    use super::*;
    const TEST_INPUT: &str = r".|...\....
|.-.\.....
.....|-...
........|.
..........
.........\
..../.\\..
.-.-/..|..
.|....-|.\
..//.|....";

    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST_INPUT), 46);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(TEST_INPUT), 51);
    }
}
