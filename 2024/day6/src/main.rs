use std::collections::HashSet;

use utils::Grid;

struct Game {
    grid: Grid<char>,
    starting_pos: (usize, usize),
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
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

    let starting_pos = grid
        .position_iter()
        .find(|(row, col)| grid.get(*row, *col).unwrap() == '^')
        .expect("can't find the starting position");

    Game { grid, starting_pos }
}

enum Solution {
    Visited(HashSet<(usize, usize)>),
    Loop(bool),
}

fn visit(game: &Game, detect_loop: bool) -> Solution {
    let mut direction = Direction::Top;
    let grid = &game.grid;
    let (mut row, mut col) = game.starting_pos;

    let mut visited = HashSet::from([game.starting_pos]);
    let mut visit_direction = HashSet::from([(game.starting_pos, direction)]);
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
                };
                if detect_loop {
                    if visit_direction.contains(&((row, col), direction)) {
                        return Solution::Loop(true);
                    }
                    visit_direction.insert(((row, col), direction));
                }
            }
            Some('.' | '^') => {
                row = next_row;
                col = next_col;
                if !detect_loop {
                    visited.insert((row, col));
                }
            }
            _ => break,
        }
    }

    if detect_loop {
        Solution::Loop(false)
    } else {
        Solution::Visited(visited)
    }
}

fn part1(input: &str) -> usize {
    let game = parse(input);
    if let Solution::Visited(visited) = visit(&game, false) {
        visited.len()
    } else {
        unreachable!()
    }
}

fn part2(input: &str) -> usize {
    let mut game = parse(input);
    let Solution::Visited(visited) = visit(&game, false) else {
        unreachable!()
    };

    let mut loops = 0;
    for (row, col) in visited {
        game.grid.set(row, col, '#');
        if let Solution::Loop(is_loop) = visit(&game, true) {
            if is_loop {
                loops += 1;
            }
        }
        game.grid.set(row, col, '.');
    }

    loops
}

fn main() {
    println!("part1={}", part1(include_str!("../input.txt")));
    println!("part2={}", part2(include_str!("../input.txt")));
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...";
    #[test]
    fn test_part1() {
        assert_eq!(part1(INPUT), 41);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(INPUT), 6);
    }
}
