use std::{collections::BinaryHeap, collections::HashSet};

fn parse_input() -> Vec<Vec<u8>> {
    include_str!("../input.txt")
        .lines()
        .map(|l| l.bytes().map(|b| b - b'0').collect())
        .collect()
}

fn neighbors<'a>(
    grid: &'a [Vec<u8>],
    i: usize,
    j: usize,
) -> impl Iterator<Item = (usize, usize)> + 'a {
    let (max_i, max_j) = (grid[..][..].len(), grid[..][0].len());
    [
        (i, j.wrapping_sub(1)),
        (i, j + 1),
        (i.wrapping_sub(1), j),
        (i + 1, j),
    ]
    .into_iter()
    .filter(move |&(i, j)| i < max_i && j < max_j)
}

fn neighbors_height<'a>(grid: &'a [Vec<u8>], i: usize, j: usize) -> impl Iterator<Item = u8> + 'a {
    neighbors(grid, i, j).map(|(i, j)| grid[i][j])
}

fn basin(grid: &[Vec<u8>], i: usize, j: usize) -> HashSet<(usize, usize)> {
    let mut basin_set = HashSet::new();
    basin_r(grid, i, j, &mut basin_set);
    basin_set
}

fn basin_r(grid: &[Vec<u8>], i: usize, j: usize, basin_set: &mut HashSet<(usize, usize)>) {
    let low_height = grid[i][j];

    for (ni, nj) in neighbors(&grid, i, j)
        .filter(move |&(ni, nj)| grid[ni][nj] != 9 && grid[ni][nj] > low_height)
    {
        basin_r(&grid, ni, nj, basin_set);
        basin_set.insert((ni, nj));
    }
}

fn part1() -> u32 {
    let grid = parse_input();

    (0..grid[..][..].len())
        .flat_map(|i| (0..grid[..][0].len()).map(move |j| (i, j)))
        .filter(|&(i, j)| neighbors_height(&grid, i, j).all(|height| height > grid[i][j]))
        .map(|(i, j)| grid[i][j] as u32 + 1)
        .sum()
}

fn part2() -> usize {
    let grid = parse_input();
    let mut largest_basin: BinaryHeap<usize> = BinaryHeap::new();

    for i in 0..grid[..][..].len() {
        for j in 0..grid[..][i].len() {
            if neighbors_height(&grid, i, j).all(|height| height > grid[i][j]) {
                let basin_set = basin(&grid, i, j);
                largest_basin.push(basin_set.len() + 1);
            }
        }
    }

    (0..3).map(|_| largest_basin.pop().unwrap()).product()
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        println!("answer: {}", part1());
    }

    #[test]
    fn test_part2() {
        println!("answer: {}", part2());
    }
}
