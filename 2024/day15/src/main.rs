use std::collections::{HashSet, VecDeque};

use utils::Grid;

#[derive(Debug)]
struct Game {
    grid: Grid<char>,
    instructions: Vec<char>,
    robot: (i32, i32),
}

fn extend(input: &str) -> String {
    input
        .chars()
        .map(|c| match c {
            'O' => "[]".to_string(),
            '@' => "@.".to_string(),
            '.' => "..".to_string(),
            '#' => "##".to_string(),
            c => c.to_string(),
        })
        .collect()
}

fn parse(input: &str) -> Game {
    let (grid, instructions) = input.split_once("\n\n").unwrap();
    let (grid, instructions) = (
        Grid::<char>::parse(grid),
        instructions.lines().flat_map(|l| l.chars()).collect(),
    );
    let robot = grid
        .position_iter()
        .find(|(row, col)| grid.get(*row, *col).unwrap() == '@')
        .unwrap();

    Game {
        grid,
        instructions,
        robot,
    }
}

fn next(instr: char, row: i32, col: i32) -> (i32, i32) {
    match instr {
        '>' => (row, col + 1),
        '<' => (row, col - 1),
        '^' => (row - 1, col),
        'v' => (row + 1, col),
        _ => unreachable!(),
    }
}

fn push(grid: &mut Grid<char>, instr: char, row: i32, col: i32) -> bool {
    let mut queue = VecDeque::from([(row, col)]);
    let mut visited = HashSet::new();
    let mut safe = true;
    let mut move_from_to = Vec::new();

    while let Some((row, col)) = queue.pop_front() {
        let (next_row, next_col) = next(instr, row, col);

        if visited.insert((row, col)) {
            match grid.get(row, col).unwrap() {
                '[' if instr == '^' || instr == 'v' => {
                    if !visited.contains(&(row, col + 1)) {
                        queue.push_back((row, col + 1));
                    }

                    queue.push_back((next_row, next_col));
                    move_from_to.push([(row, col), (next_row, next_col)]);
                }
                ']' if instr == '^' || instr == 'v' => {
                    if !visited.contains(&(row, col - 1)) {
                        queue.push_back((row, col - 1));
                    }
                    queue.push_back((next_row, next_col));
                    move_from_to.push([(row, col), (next_row, next_col)]);
                }
                'O' | '[' | ']' => {
                    queue.push_back((next_row, next_col));
                    move_from_to.push([(row, col), (next_row, next_col)]);
                }
                '.' => {}
                '#' => {
                    safe = false;
                    break;
                }
                _ => unreachable!(),
            }
        }
    }

    if safe {
        for [(from_row, from_col), (to_row, to_col)] in move_from_to.into_iter().rev() {
            grid.set(to_row, to_col, grid.get(from_row, from_col).unwrap())
                .unwrap();
            grid.set(from_row, from_col, '.').unwrap();
        }
    }

    safe
}

fn solve(input: &str) -> usize {
    let Game {
        mut grid,
        instructions,
        robot,
    } = parse(input);

    let (mut robot_row, mut robot_col) = robot;
    for instr in instructions {
        let (next_row, next_col) = next(instr, robot_row, robot_col);

        (robot_row, robot_col) = match grid.get(next_row, next_col).unwrap() {
            '#' => (robot_row, robot_col),
            '.' => {
                grid.set(robot_row, robot_col, '.').unwrap();
                (next_row, next_col)
            }
            'O' | '[' | ']' => {
                if push(&mut grid, instr, next_row, next_col) {
                    grid.set(robot_row, robot_col, '.').unwrap();
                    (next_row, next_col)
                } else {
                    (robot_row, robot_col)
                }
            }
            _ => unreachable!(),
        };

        grid.set(robot_row, robot_col, '@').unwrap();
    }

    grid.position_iter()
        .filter(|(row, col)| {
            let c = grid.get(*row, *col).unwrap();
            c == 'O' || c == '['
        })
        .map(|(row, col)| row * 100 + col)
        .sum::<i32>() as usize
}

fn part1(input: &str) -> usize {
    solve(input)
}

fn part2(input: &str) -> usize {
    let input = extend(input);
    solve(&input)
}

fn main() {
    println!("part1={}", part1(include_str!("../input.txt")));
    println!("part2={}", part2(include_str!("../input.txt")));
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = "##########
#..O..O.O#
#......O.#
#.OO..O.O#
#..O@..O.#
#O#..O...#
#O..O..O.#
#.OO.O.OO#
#....O...#
##########

<vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
<<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
>^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
<><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^";

    #[test]
    fn test_part1_simple() {
        let input = "########
#..O.O.#
##@.O..#
#...O..#
#.#.O..#
#...O..#
#......#
########

<^^>>>vv<v>>v<<";
        part1(input);
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(INPUT), 10092);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(INPUT), 9021);
    }
}
