use std::collections::{HashMap, HashSet};

struct Grid(Vec<Vec<u8>>);

struct Part {
    number: usize,
    i: usize,
    j: usize,
}

impl Grid {
    fn get(&self, i: usize, j: usize) -> u8 {
        self.0[j][i]
    }

    fn max_i(&self) -> usize {
        self.0[..][..].len()
    }

    fn max_j(&self) -> usize {
        self.0[..].len()
    }

    fn neighbors(&self, i: usize, j: usize) -> impl Iterator<Item = (usize, usize)> {
        let (max_i, max_j) = (self.max_i(), self.max_j());
        [
            (i + 1, j),
            (i.wrapping_sub(1), j),
            (i, j + 1),
            (i, j.wrapping_sub(1)),
            (i.wrapping_sub(1), j.wrapping_sub(1)),
            (i + 1, j + 1),
            (i.wrapping_sub(1), j + 1),
            (i + 1, j.wrapping_sub(1)),
        ]
        .into_iter()
        .filter(move |&(i, j)| i < max_i && j < max_j)
    }
}

fn parse(input: &str) -> Grid {
    Grid(input.lines().map(|l| l.bytes().collect()).collect())
}

fn is_symbol(c: u8) -> bool {
    !c.is_ascii_alphanumeric() && c != b'.'
}

// trick to get a number's number of digits
fn number_len(number: usize) -> usize {
    (number.checked_ilog10().unwrap_or(0) + 1) as usize
}

fn number_with_adj_symbol(grid: &Grid, i: usize, j: usize) -> Option<usize> {
    let mut x = i;
    let mut has_neighbor_symbol = false;

    while x < grid.max_i() && grid.get(x, j).is_ascii_digit() {
        has_neighbor_symbol = has_neighbor_symbol
            || grid
                .neighbors(x, j)
                .map(|(i, j)| grid.get(i, j))
                .any(is_symbol);
        x += 1;
    }

    if has_neighbor_symbol {
        let number = unsafe { std::str::from_utf8_unchecked(&grid.0[j][i..x]) }
            .parse()
            .unwrap();
        Some(number)
    } else {
        None
    }
}

fn get_parts(grid: &Grid) -> Vec<Part> {
    let mut parts: Vec<Part> = vec![];

    for j in 0..grid.max_j() {
        let mut i = 0;
        while i < grid.max_i() {
            if grid.get(i, j).is_ascii_digit() {
                if let Some(number) = number_with_adj_symbol(grid, i, j) {
                    parts.push(Part { number, i, j });
                    i += number_len(number);
                } else {
                    i += 1;
                }
            } else {
                i += 1;
            }
        }
    }

    parts
}

fn part1(input: &str) -> usize {
    let grid = parse(input);
    get_parts(&grid).into_iter().map(|part| part.number).sum()
}

fn part2(input: &str) -> usize {
    let grid = parse(input);
    let parts = get_parts(&grid);
    let mut parts_map: HashMap<(usize, usize), usize> = HashMap::new();
    let mut gear_ratios = 0;

    for part in parts {
        for size in 0..number_len(part.number) {
            parts_map.insert((part.i + size, part.j), part.number);
        }
    }

    for i in 0..grid.max_i() {
        for j in 0..grid.max_i() {
            let c = grid.get(i, j);
            if is_symbol(c) {
                let mut gears = HashSet::new();
                for neigh in grid.neighbors(i, j) {
                    if let Some(&number) = parts_map.get(&neigh) {
                        gears.insert(number);
                    }
                }
                if gears.len() == 2 {
                    gear_ratios += gears.iter().product::<usize>();
                }
            }
        }
    }

    gear_ratios
}

fn main() {
    println!("part1={}", part1(include_str!("../input.txt")));
    println!("part2={}", part2(include_str!("../input.txt")));
}

#[cfg(test)]
mod tests {
    use super::*;
    const TEST_INPUT: &str = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";

    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST_INPUT), 4361);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(TEST_INPUT), 467835);
    }
}
