use itertools::Itertools;
use std::collections::{HashMap, HashSet};

use utils::Grid;

struct Game {
    grid: Grid<char>,
    antennas: HashMap<char, Vec<(i32, i32)>>,
}

fn parse(input: &str) -> Game {
    let mut columns = 0;
    let grid = input
        .lines()
        .flat_map(|line| {
            columns = line.len() as i32;
            line.chars()
        })
        .collect::<Vec<_>>();
    let rows = grid.len() as i32 / columns;

    let grid = Grid {
        grid,
        rows,
        columns,
    };

    let antennas = grid
        .position_iter()
        .filter_map(|(row, col)| {
            let c = grid.get(row, col).unwrap();

            if c != '.' {
                Some((c, (row, col)))
            } else {
                None
            }
        })
        .fold(HashMap::new(), |mut acc, (c, pos)| {
            acc.entry(c)
                .and_modify(|v: &mut Vec<_>| v.push(pos))
                .or_insert(Vec::from([pos]));
            acc
        });

    Game { grid, antennas }
}

fn part1(input: &str) -> usize {
    let Game { grid, antennas } = parse(input);

    let mut antinodes = HashSet::new();
    for antenna in antennas.values() {
        let antennas_combinations = antenna.iter().combinations(2);

        for combination in antennas_combinations {
            let (antenna1, antenna2) = (combination[0], combination[1]);
            let diff_row = antenna2.0.abs_diff(antenna1.0) as i32;
            let diff_col = antenna2.1.abs_diff(antenna1.1) as i32;

            let mut antinode1 = (0, 0);
            let mut antinode2 = (0, 0);
            if antenna1.0 < antenna2.0 {
                antinode1.0 = antenna1.0 - diff_row;
                antinode2.0 = antenna2.0 + diff_row;
            } else {
                antinode1.0 = antenna1.0 + diff_row;
                antinode2.0 = antenna2.0 - diff_row;
            }

            if antenna1.1 < antenna2.1 {
                antinode1.1 = antenna1.1 - diff_col;
                antinode2.1 = antenna2.1 + diff_col;
            } else {
                antinode1.1 = antenna1.1 + diff_col;
                antinode2.1 = antenna2.1 - diff_col;
            }

            if grid.get(antinode1.0, antinode1.1).is_some() {
                antinodes.insert(antinode1);
            }

            if grid.get(antinode2.0, antinode2.1).is_some() {
                antinodes.insert(antinode2);
            }
        }
    }

    antinodes.len()
}

fn part2(input: &str) -> usize {
    let Game { grid, antennas } = parse(input);

    let mut antinodes = HashSet::new();
    for antenna in antennas.values() {
        let antennas_combinations = antenna.iter().combinations(2);

        for combination in antennas_combinations {
            let (antenna1, antenna2) = (combination[0], combination[1]);
            let diff_row = antenna2.0.abs_diff(antenna1.0) as i32;
            let diff_col = antenna2.1.abs_diff(antenna1.1) as i32;

            let mut vec1 = (0, 0);
            let mut vec2 = (0, 0);
            if antenna1.0 < antenna2.0 {
                vec1.0 = -diff_row;
                vec2.0 = diff_row;
            } else {
                vec1.0 = diff_row;
                vec2.0 = -diff_row;
            }

            if antenna1.1 < antenna2.1 {
                vec1.1 = -diff_col;
                vec2.1 = diff_col;
            } else {
                vec1.1 = diff_col;
                vec2.1 = -diff_col;
            }

            let mut antinode = *antenna1;
            loop {
                if grid.get(antinode.0, antinode.1).is_some() {
                    antinodes.insert(antinode);
                } else {
                    break;
                }
                antinode = (antinode.0 + vec1.0, antinode.1 + vec1.1);
            }

            let mut antinode = *antenna2;
            loop {
                if grid.get(antinode.0, antinode.1).is_some() {
                    antinodes.insert(antinode);
                } else {
                    break;
                }
                antinode = (antinode.0 + vec2.0, antinode.1 + vec2.1);
            }
        }
    }

    antinodes.len()
}

fn main() {
    println!("part1={}", part1(include_str!("../input.txt")));
    println!("part2={}", part2(include_str!("../input.txt")));
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = "............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............";

    #[test]
    fn test_part1() {
        assert_eq!(part1(INPUT), 14);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(INPUT), 34);
    }

    #[test]
    fn test_part2_simple() {
        let input = "T.........
...T......
.T........
..........
..........
..........
..........
..........
..........
..........";
        assert_eq!(part2(input), 9);
    }
}
