use std::collections::HashSet;

use utils::Grid;

struct Game {
    grid: Grid<char>,
    starting_pos: (usize, usize),
}

enum Direction {
    Top,
    Right,
    Bottom,
    Left,
}

fn parse(input: &str) -> Game {
    let mut columns = 0;
    let grid = input
        .lines()
        .flat_map(|line| {
            columns = line.len();
            line.chars()
        })
        .collect::<Vec<_>>();
    let rows = grid.len() / columns;

    let grid = Grid {
        grid,
        rows,
        columns,
    };

    // FIXME: move to utils
    let starting_pos = (0..rows)
        .flat_map(|row| (0..columns).map(move |col| (row, col)))
        .find(|(row, col)| grid.get(*row, *col).unwrap() == '^')
        .expect("can't find the starting position");

    Game { grid, starting_pos }
}

fn part1(input: &str) -> usize {
    let game = parse(input);

    let mut direction = Direction::Top;
    let (grid, (mut row, mut col)) = (game.grid, (game.starting_pos));

    let mut visited = HashSet::from([game.starting_pos]);
    loop {
        let (next_row, next_col) = match direction {
            Direction::Top if row >= 1 => (row - 1, col),
            Direction::Right if col < grid.columns - 1 => (row, col + 1),
            Direction::Bottom if row < grid.rows - 1 => (row + 1, col),
            Direction::Left if col >= 1 => (row, col - 1),
            _ => break,
        };

        match grid.get(next_row, next_col) {
            Some('#') => {
                direction = match direction {
                    Direction::Top => Direction::Right,
                    Direction::Right => Direction::Bottom,
                    Direction::Bottom => Direction::Left,
                    Direction::Left => Direction::Top,
                }
            }
            Some('.' | '^') => {
                row = next_row;
                col = next_col;
                visited.insert((row, col));
            }
            _ => break,
        }
    }

    visited.len()
}

fn part2(input: &str) -> usize {
    0
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
        let input = "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...";
        assert_eq!(part1(input), 41);
    }

    #[test]
    fn test_part2() {
        let input = "";
        assert_eq!(part2(input), 31);
    }
}
