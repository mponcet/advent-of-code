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
}

fn parse(input: &str) -> Vec<Grid> {
    input
        .split("\n\n")
        .map(|grid| {
            let columns = grid.lines().next().unwrap().len();
            let rows = grid.lines().count();
            Grid {
                grid: grid.lines().flat_map(|l| l.bytes()).collect(),
                columns,
                rows,
            }
        })
        .collect()
}

fn vertical_symmetry(grid: &Grid, (left, right): (usize, usize), with_smudge: bool) -> bool {
    let width = left.min(grid.columns - right - 1);
    let cnt: usize = (0..grid.rows)
        .map(|j| {
            (0..=width)
                .filter(|w| grid.get(left - w, j) == grid.get(right + w, j))
                .count()
        })
        .sum();

    if with_smudge {
        (width + 1) * grid.rows - 1 == cnt
    } else {
        cnt == (width + 1) * grid.rows
    }
}

fn horizontal_symmetry(grid: &Grid, (top, bottom): (usize, usize), with_smudge: bool) -> bool {
    let width = top.min(grid.rows - bottom - 1);
    let cnt: usize = (0..grid.columns)
        .map(|i| {
            (0..=width)
                .filter(|w| grid.get(i, top - w) == grid.get(i, bottom + w))
                .count()
        })
        .sum();

    if with_smudge {
        (width + 1) * grid.columns - 1 == cnt
    } else {
        cnt == (width + 1) * grid.columns
    }
}

fn parts(input: &str, with_smudge: bool) -> usize {
    parse(input)
        .into_iter()
        .map(|grid| {
            let left_cols = (0..grid.columns - 1)
                .find_map(|i| {
                    if vertical_symmetry(&grid, (i, i + 1), with_smudge) {
                        Some(i + 1)
                    } else {
                        None
                    }
                })
                .unwrap_or(0);
            let above_rows = (0..grid.rows - 1)
                .find_map(|j| {
                    if horizontal_symmetry(&grid, (j, j + 1), with_smudge) {
                        Some(j + 1)
                    } else {
                        None
                    }
                })
                .unwrap_or(0);
            left_cols + 100 * above_rows
        })
        .sum()
}

fn main() {
    println!("part1={}", parts(include_str!("../input.txt"), false));
    println!("part2={}", parts(include_str!("../input.txt"), true));
}

#[cfg(test)]
mod tests {
    use super::*;
    const TEST_INPUT: &str = "#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#";

    #[test]
    fn test_part1() {
        assert_eq!(parts(TEST_INPUT, false), 405);
    }

    #[test]
    fn test_part2() {
        assert_eq!(parts(TEST_INPUT, true), 400);
    }
}
