use utils::{DiagonalDirection, Grid};

fn parse(input: &str) -> Grid<char> {
    let mut columns = 0;
    let grid: Vec<_> = input
        .lines()
        .flat_map(|l| {
            columns = l.len() as i32;
            l.chars()
        })
        .collect();

    let rows = grid.len() as i32 / columns;

    Grid {
        grid,
        columns,
        rows,
    }
}

fn part1(input: &str) -> usize {
    let grid = parse(input);
    let mut nr_xmas = 0;

    for col in 0..grid.rows {
        for row in 0..grid.columns {
            // horizontal
            if let Some(row) = grid.row_slice(row, col..col + 4) {
                let s = row.iter().collect::<String>();
                if s == "XMAS" || s == "SAMX" {
                    nr_xmas += 1;
                }
            }

            // vertical
            if let Some(col) = grid.col_slice(col, row..row + 4) {
                let s = col.iter().collect::<String>();
                if s == "XMAS" || s == "SAMX" {
                    nr_xmas += 1;
                }
            }

            // diagonal
            if let Some(diagonal) = grid.diagonal_from(row, col, DiagonalDirection::BottomRight(4))
            {
                let s = diagonal.iter().collect::<String>();
                if s == "XMAS" || s == "SAMX" {
                    nr_xmas += 1;
                }
            }

            if let Some(diagonal) = grid.diagonal_from(row, col, DiagonalDirection::TopRight(4)) {
                let s = diagonal.iter().collect::<String>();
                if s == "XMAS" || s == "SAMX" {
                    nr_xmas += 1;
                }
            }
        }
    }

    nr_xmas
}

fn part2(input: &str) -> usize {
    let grid = parse(input);
    let mut nr_xmas = 0;

    for col in 0..grid.rows {
        for row in 0..grid.columns {
            if let Some(c) = grid.get(row, col) {
                if c == 'A' {
                    let diagonal1 = [
                        (row.wrapping_sub(1), col.wrapping_sub(1)),
                        (row + 1, col + 1),
                    ]
                    .into_iter()
                    .filter_map(|(row, col)| grid.get(row, col))
                    .collect::<String>();
                    let diagonal2 = [
                        (row + 1, col.wrapping_sub(1)),
                        (row.wrapping_sub(1), col + 1),
                    ]
                    .into_iter()
                    .filter_map(|(row, col)| grid.get(row, col))
                    .collect::<String>();

                    if (diagonal1 == "SM" || diagonal1 == "MS")
                        && (diagonal2 == "SM" || diagonal2 == "MS")
                    {
                        nr_xmas += 1;
                    }
                }
            }
        }
    }

    nr_xmas
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
        let input = "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";
        assert_eq!(part1(input), 18);
    }

    #[test]
    fn test_part2() {
        let input = ".M.S......
..A..MSMS.
.M.S.MAA..
..A.ASMSM.
.M.S.M....
..........
S.S.S.S.S.
.A.A.A.A..
M.M.M.M.M.
..........";
        assert_eq!(part2(input), 9);
    }
}
