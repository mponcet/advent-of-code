use std::collections::{HashMap, VecDeque};

#[derive(Debug)]
struct Grid {
    grid: Vec<char>,
    start: (usize, usize),
    columns: usize,
    rows: usize,
}

impl Grid {
    fn get(&self, (i, j): (usize, usize)) -> Option<char> {
        if i < self.columns && j < self.rows {
            self.grid.get(j * self.columns + i).copied()
        } else {
            None
        }
    }

    fn neighbors(&self, (i, j): (usize, usize)) -> impl Iterator<Item = (usize, usize)> + '_ {
        [
            (i + 1, j),
            (i.wrapping_sub(1), j),
            (i, j + 1),
            (i, j.wrapping_sub(1)),
        ]
        .into_iter()
        .filter(move |&(x, y)| x < self.columns && y < self.rows)
    }

    fn get_infinity(&self, (i, j): (isize, isize)) -> char {
        let i = (i % self.columns as isize + self.columns as isize) as usize % self.columns;
        let j = (j % self.columns as isize + self.columns as isize) as usize % self.columns;
        self.grid.get(j * self.columns + i).copied().unwrap()
    }

    fn neighbors_infinity(
        &self,
        (i, j): (isize, isize),
    ) -> impl Iterator<Item = (isize, isize)> + '_ {
        [(i + 1, j), (i - 1, j), (i, j + 1), (i, j - 1)].into_iter()
    }
}

fn parse(input: &str) -> Grid {
    let mut columns = 0;
    let grid: Vec<_> = input
        .lines()
        .flat_map(|l| {
            columns = l.len();
            l.chars()
        })
        .collect();

    let rows = grid.len() / columns;

    let start = grid
        .iter()
        .enumerate()
        .find(|(_, &c)| c == 'S')
        .map(|(idx, _)| idx)
        .unwrap();

    Grid {
        grid,
        start: (start % columns, start / rows),
        columns,
        rows,
    }
}

fn bfs(input: &str, max_steps: usize) -> usize {
    let grid = parse(input);
    let mut frontier = VecDeque::from([(grid.start, 0)]);
    let mut costs: HashMap<(usize, usize), usize> = HashMap::new();

    while let Some((pos, steps)) = frontier.pop_front() {
        if let Some(&cost) = costs.get(&pos) {
            if cost == steps {
                continue;
            }
        }
        costs.insert(pos, steps);
        if steps == max_steps {
            continue;
        }
        for neigh in grid.neighbors(pos) {
            match grid.get(neigh).unwrap() {
                '.' | 'S' => frontier.push_back((neigh, steps + 1)),
                '#' => {}
                _ => unreachable!(),
            }
        }
    }

    costs.values().filter(|&&c| c == max_steps).count()
}

fn part1(input: &str, max_steps: usize) -> usize {
    bfs(input, max_steps)
}

fn bfs_infinity(input: &str, max_steps: usize) -> usize {
    let grid = parse(input);
    let start = (grid.start.0 as isize, grid.start.1 as isize);
    let mut frontier = VecDeque::from([(start, 0)]);
    let mut costs: HashMap<(isize, isize), usize> = HashMap::new();

    while let Some((pos, steps)) = frontier.pop_front() {
        if let Some(&cost) = costs.get(&pos) {
            if cost == steps {
                continue;
            }
        }
        costs.insert(pos, steps);
        if steps == max_steps {
            continue;
        }
        for neigh in grid.neighbors_infinity(pos) {
            match grid.get_infinity(neigh) {
                '.' | 'S' => frontier.push_back((neigh, steps + 1)),
                '#' => {}
                _ => unreachable!(),
            }
        }
    }

    costs.values().filter(|&&c| c == max_steps).count()
}

fn part2(input: &str) -> usize {
    let a0 = bfs_infinity(input, 65);
    let a1 = bfs_infinity(input, 65 + 131);
    let a2 = bfs_infinity(input, 65 + 131 * 2);
    println!("{a0} {a1} {a2}");
    // quadratic equation: 3710 + 14685 x + 14581 x^2

    let steps: usize = 26501365 / 131;

    steps.pow(2) * 14581 + steps * 14685 + 3710
}

fn main() {
    println!("part1={}", part1(include_str!("../input.txt"), 64));
    println!("part2={}", part2(include_str!("../input.txt")));
}

#[cfg(test)]
mod tests {
    use super::*;
    const TEST_INPUT: &str = "...........
.....###.#.
.###.##..#.
..#.#...#..
....#.#....
.##..S####.
.##..#...#.
.......##..
.##.#.####.
.##..##.##.
...........";

    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST_INPUT, 6), 16);
    }
}
