use itertools::Itertools;
use std::collections::{HashMap, HashSet};

use utils::Grid;

struct Game {
    grid: Grid<char>,
    antennas: HashMap<char, Vec<(i32, i32)>>,
}

fn parse(input: &str) -> Game {
    let grid = Grid::parse(input);

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
    for positions in antennas.values() {
        let permutations = positions.iter().permutations(2);

        for permutation in permutations {
            let (antenna1, antenna2) = (permutation[0], permutation[1]);
            let diff_row = antenna2.0 - antenna1.0;
            let diff_col = antenna2.1 - antenna1.1;

            let antinode = (antenna2.0 + diff_row, antenna2.1 + diff_col);
            if grid.get(antinode.0, antinode.1).is_some() {
                antinodes.insert(antinode);
            }
        }
    }

    antinodes.len()
}

fn part2(input: &str) -> usize {
    let Game { grid, antennas } = parse(input);

    let mut antinodes = HashSet::new();
    for positions in antennas.values() {
        let permutations = positions.iter().permutations(2);

        for permutation in permutations {
            let (antenna1, antenna2) = (permutation[0], permutation[1]);
            let diff_row = antenna2.0 - antenna1.0;
            let diff_col = antenna2.1 - antenna1.1;

            let mut antinode = *antenna2;
            loop {
                if grid.get(antinode.0, antinode.1).is_some() {
                    antinodes.insert(antinode);
                } else {
                    break;
                }
                antinode = (antinode.0 + diff_row, antinode.1 + diff_col);
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
