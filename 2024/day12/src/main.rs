use std::collections::{HashSet, VecDeque};

use utils::Grid;

fn bfs(grid: &Grid<char>, row: i32, col: i32) -> Vec<(i32, i32)> {
    let color = grid.get(row, col).unwrap();
    let mut explore = VecDeque::from([(row, col)]);
    let mut visited = HashSet::from([(row, col)]);
    let mut region = Vec::from([(row, col)]);

    while let Some((row, col)) = explore.pop_front() {
        for (n_row, n_col) in grid.neighbors_cross(row, col) {
            let n_color = grid.get(n_row, n_col).unwrap();
            if !visited.contains(&(n_row, n_col)) && n_color == color {
                explore.push_back((n_row, n_col));
                region.push((n_row, n_col));
            }
            visited.insert((n_row, n_col));
        }
    }

    region
}

fn perimeter(grid: &Grid<char>, region: &[(i32, i32)]) -> usize {
    let color = grid.get(region[0].0, region[0].1).unwrap();
    region
        .iter()
        .flat_map(|(row, col)| {
            grid.neighbors_cross_unchecked(*row, *col)
                .filter(|&(n_row, n_col)| {
                    grid.get(n_row, n_col).is_none() || grid.get(n_row, n_col).unwrap() != color
                })
        })
        .count()
}

fn sides(region: &[(i32, i32)]) -> usize {
    let region_set = region.iter().collect::<HashSet<_>>();
    region
        .iter()
        .map(|(row, col)| {
            let mut count = 0;
            // count convex corners
            count += [
                [(0, -1), (-1, 0)],
                [(0, 1), (-1, 0)],
                [(1, 0), (0, 1)],
                [(1, 0), (0, -1)],
            ]
            .into_iter()
            .filter(|v| {
                v.iter()
                    .all(|d| !region_set.contains(&(row + d.0, col + d.1)))
            })
            .count();

            // count concave corners
            // top left
            if [(0, -1), (-1, 0)]
                .into_iter()
                .all(|d| region_set.contains(&(row + d.0, col + d.1)))
                && !region_set.contains(&(*row - 1, *col - 1))
            {
                count += 1
            }
            // top right
            if [(0, 1), (-1, 0)]
                .into_iter()
                .all(|d| region_set.contains(&(row + d.0, col + d.1)))
                && !region_set.contains(&(*row - 1, *col + 1))
            {
                count += 1
            }
            // bottom right
            if [(1, 0), (0, 1)]
                .into_iter()
                .all(|d| region_set.contains(&(row + d.0, col + d.1)))
                && !region_set.contains(&(*row + 1, *col + 1))
            {
                count += 1
            }
            // bottom left
            if [(1, 0), (0, -1)]
                .into_iter()
                .all(|d| region_set.contains(&(row + d.0, col + d.1)))
                && !region_set.contains(&(*row + 1, *col - 1))
            {
                count += 1
            }
            count
        })
        .sum()
}

fn part1(input: &str) -> usize {
    let grid = Grid::<char>::parse(input);
    let mut visited = HashSet::<(i32, i32)>::new();

    grid.position_iter()
        .map(|(row, col)| {
            if !visited.contains(&(row, col)) {
                let region = bfs(&grid, row, col);
                visited.extend(region.iter());
                region.len() * perimeter(&grid, &region)
            } else {
                0
            }
        })
        .sum()
}

fn part2(input: &str) -> usize {
    let grid = Grid::<char>::parse(input);
    let mut visited = HashSet::<(i32, i32)>::new();

    grid.position_iter()
        .map(|(row, col)| {
            if !visited.contains(&(row, col)) {
                let region = bfs(&grid, row, col);
                visited.extend(region.iter());
                region.len() * sides(&region)
            } else {
                0
            }
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
    const INPUT: &str = "RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE";

    #[test]
    fn test_part1() {
        assert_eq!(part1(INPUT), 1930);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(INPUT), 1206);
    }
    #[test]
    fn test_part2_simple() {
        let input = "AAAAAA
AAABBA
AAABBA
ABBAAA
ABBAAA
AAAAAA";
        assert_eq!(part2(input), 368);
    }
}
