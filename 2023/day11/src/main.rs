use std::collections::HashSet;

use itertools::Itertools;

fn parse(input: &str) -> (Vec<(usize, usize)>, HashSet<usize>, HashSet<usize>) {
    let line_len = input.lines().next().unwrap().len();
    let galaxies = input
        .lines()
        .enumerate()
        .flat_map(|(j, l)| {
            l.bytes()
                .enumerate()
                .filter_map(move |(i, c)| if c == b'#' { Some((i, j)) } else { None })
        })
        .collect::<Vec<_>>();

    let empty_columns: HashSet<_> = (0..line_len)
        .filter(|i| galaxies.iter().all(|(gi, _)| i != gi))
        .collect();
    let empty_rows: HashSet<_> = (0..line_len)
        .filter(|j| galaxies.iter().all(|(_, gj)| j != gj))
        .collect();

    (galaxies, empty_columns, empty_rows)
}

fn parts(input: &str, expansion: usize) -> usize {
    let (galaxies, empty_columns, empty_rows) = parse(input);

    galaxies
        .iter()
        .tuple_combinations()
        .map(|((i1, j1), (i2, j2))| {
            let (&min_i, &(mut max_i)) = (i1.min(i2), i1.max(i2));
            let (&min_j, &(mut max_j)) = (j1.min(j2), j1.max(j2));

            let count_empty_columns = (min_i..=max_i)
                .filter(|i| empty_columns.contains(i))
                .count();
            max_i += count_empty_columns * (expansion - 1);

            let count_empty_rows = (min_j..=max_j).filter(|j| empty_rows.contains(j)).count();
            max_j += count_empty_rows * (expansion - 1);

            (max_i - min_i) + (max_j - min_j)
        })
        .sum()
}

fn main() {
    println!("part1={}", parts(include_str!("../input.txt"), 2));
    println!("part2={}", parts(include_str!("../input.txt"), 1000000));
}

#[cfg(test)]
mod tests {
    use super::*;
    const TEST_INPUT: &str = "...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....";

    #[test]
    fn test_part1() {
        assert_eq!(parts(TEST_INPUT, 2), 374);
    }

    #[test]
    fn test_part2() {
        assert_eq!(parts(TEST_INPUT, 100), 8410);
    }
}
