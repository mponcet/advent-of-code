use std::collections::HashMap;

use utils::Grid;

fn trailheads(grid: &Grid<u8>, discovered: &mut HashMap<(i32, i32), usize>, row: i32, col: i32) {
    let (mut row, mut col) = (row, col);

    loop {
        let height = grid.get(row, col).unwrap();
        if height == b'9' {
            discovered
                .entry((row, col))
                .and_modify(|e| *e += 1)
                .or_insert(1);
            return;
        }

        let neighs = grid
            .neighbors_cross(row, col)
            .filter(|&(n_row, n_col)| {
                let n_height = grid.get(n_row, n_col).unwrap();
                n_height == height + 1
            })
            .collect::<Vec<_>>();

        if neighs.is_empty() {
            return;
        } else if neighs.len() == 1 {
            (row, col) = neighs[0];
        } else {
            neighs
                .into_iter()
                .map(|(n_row, n_col)| trailheads(grid, discovered, n_row, n_col))
                .for_each(|_| {});
            return;
        }
    }
}

fn part1(input: &str) -> usize {
    let grid = Grid::<u8>::parse(input);
    let mut discovered = HashMap::new();
    grid.position_iter()
        .filter(|&(row, col)| grid.get(row, col).unwrap() == b'0')
        .map(|(row, col)| {
            trailheads(&grid, &mut discovered, row, col);
            discovered.drain().count()
        })
        .sum()
}

fn part2(input: &str) -> usize {
    let grid = Grid::<u8>::parse(input);
    let mut discovered = HashMap::new();
    grid.position_iter()
        .filter(|&(row, col)| grid.get(row, col).unwrap() == b'0')
        .map(|(row, col)| {
            trailheads(&grid, &mut discovered, row, col);
            discovered.drain().map(|(_, count)| count).sum::<usize>()
        })
        .sum()
}

fn main() {
    println!("part1={}", part1(include_str!("../input.txt")));
    println!("part2={}", part2(include_str!("../input.txt")));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = "89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732";
        assert_eq!(part1(input), 36);
    }

    #[test]
    fn test_part2() {
        let input = ".....0.
..4321.
..5..2.
..6543.
..7..4.
..8765.
..9....";
        assert_eq!(part2(input), 3);
    }
}
