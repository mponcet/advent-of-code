use std::collections::HashMap;

#[derive(Debug)]
struct Grid {
    grid: Vec<u8>,
    columns: usize,
    rows: usize,
}

impl Grid {
    fn get(&self, i: usize, j: usize) -> u8 {
        self.grid[j * self.columns + i]
    }

    fn set(&mut self, i: usize, j: usize, value: u8) {
        self.grid[j * self.columns + i] = value;
    }
}

impl std::fmt::Display for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in self.grid.chunks_exact(self.columns) {
            let s = String::from_utf8(row.to_owned()).unwrap();
            writeln!(f, "{}", s)?
        }

        Ok(())
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

enum Direction {
    North,
    South,
    West,
    East,
}

fn tilt(grid: &mut Grid, direction: Direction) {
    match direction {
        Direction::North => {
            for i in 0..grid.columns {
                for j in 1..grid.rows {
                    let value = grid.get(i, j);

                    if value == b'O' {
                        let delta_j = (0..=j - 1)
                            .rev()
                            .take_while(|&j_backward| grid.get(i, j_backward) == b'.')
                            .count();

                        if delta_j != 0 {
                            grid.set(i, j - delta_j, b'O');
                            grid.set(i, j, b'.');
                        }
                    }
                }
            }
        }
        Direction::South => {
            for i in 0..grid.columns {
                for j in (0..grid.rows - 1).rev() {
                    let value = grid.get(i, j);

                    if value == b'O' {
                        let delta_j = (j + 1..grid.rows)
                            .take_while(|&j_upward| grid.get(i, j_upward) == b'.')
                            .count();

                        if delta_j != 0 {
                            grid.set(i, j + delta_j, b'O');
                            grid.set(i, j, b'.');
                        }
                    }
                }
            }
        }
        Direction::West => {
            for j in 0..grid.rows {
                for i in 1..grid.columns {
                    let value = grid.get(i, j);

                    if value == b'O' {
                        let delta_i = (0..=i - 1)
                            .rev()
                            .take_while(|&i_backward| grid.get(i_backward, j) == b'.')
                            .count();

                        if delta_i != 0 {
                            grid.set(i - delta_i, j, b'O');
                            grid.set(i, j, b'.');
                        }
                    }
                }
            }
        }
        Direction::East => {
            for j in 0..grid.rows {
                for i in (0..grid.columns - 1).rev() {
                    let value = grid.get(i, j);

                    if value == b'O' {
                        let delta_i = (i + 1..grid.columns)
                            .take_while(|&i_upward| grid.get(i_upward, j) == b'.')
                            .count();
                        if delta_i != 0 {
                            grid.set(i + delta_i, j, b'O');
                            grid.set(i, j, b'.');
                        }
                    }
                }
            }
        }
    }
}

fn load(grid: &Grid) -> usize {
    (1..=grid.rows)
        .rev()
        .zip(grid.grid.chunks_exact(grid.columns))
        .map(|(cnt, row)| row.iter().filter(|&&c| c == b'O').count() * cnt)
        .sum()
}

fn part1(input: &str) -> usize {
    let mut grid = parse(input);
    tilt(&mut grid, Direction::North);
    load(&grid)
}

fn part2(input: &str) -> usize {
    let mut grid = parse(input);
    let mut cycle_detector: HashMap<Vec<u8>, Vec<usize>> = HashMap::new();

    for i in 0..300 {
        tilt(&mut grid, Direction::North);
        tilt(&mut grid, Direction::West);
        tilt(&mut grid, Direction::South);
        tilt(&mut grid, Direction::East);

        let grid_content = grid.grid.to_vec();
        cycle_detector
            .entry(grid_content)
            .and_modify(|cycles| cycles.push(i))
            .or_default();
    }

    let (start, cycle) = cycle_detector
        .values()
        .filter(|cycles| cycles.len() >= 2)
        .map(|cycles| (cycles[0], cycles[1] - cycles[0]))
        .next()
        .unwrap();

    let loops = start + (1000000000 - start) % cycle;

    let mut grid = parse(input);
    for _ in 0..loops {
        tilt(&mut grid, Direction::North);
        tilt(&mut grid, Direction::West);
        tilt(&mut grid, Direction::South);
        tilt(&mut grid, Direction::East);
    }

    load(&grid)
}

fn main() {
    println!("part1={}", part1(include_str!("../input.txt")));
    println!("part2={}", part2(include_str!("../input.txt")));
}

#[cfg(test)]
mod tests {
    use super::*;
    const TEST_INPUT: &str = "O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....";

    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST_INPUT), 136);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(TEST_INPUT), 64);
    }
}
