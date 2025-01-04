use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap, HashSet},
};

use utils::Grid;

struct Game {
    grid: Grid<char>,
    start: (i32, i32),
    end: (i32, i32),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    fn coords(self) -> (i32, i32) {
        match self {
            Direction::North => (-1, 0),
            Direction::East => (0, 1),
            Direction::South => (1, 0),
            Direction::West => (0, -1),
        }
    }
}

fn next(
    grid: &Grid<char>,
    (row, col): (i32, i32),
    direction: Direction,
) -> impl Iterator<Item = ((i32, i32), Direction)> + '_ {
    let neighs = match direction {
        Direction::North => [Direction::East, Direction::North, Direction::West],
        Direction::East => [Direction::South, Direction::East, Direction::North],
        Direction::South => [Direction::West, Direction::South, Direction::East],
        Direction::West => [Direction::North, Direction::West, Direction::South],
    };
    neighs
        .into_iter()
        .map(|direction| (direction, direction.coords()))
        .map(move |(direction, (r, c))| ((row + r, col + c), direction))
        .filter(|((row, col), _)| grid.get(*row, *col).is_some_and(|c| c != '#'))
}

fn parse(input: &str) -> Game {
    let grid = Grid::<char>::parse(input);
    let start = grid
        .position_iter()
        .find(|(row, col)| grid.get(*row, *col).unwrap() == 'S')
        .unwrap();
    let end = grid
        .position_iter()
        .find(|(row, col)| grid.get(*row, *col).unwrap() == 'E')
        .unwrap();

    Game { grid, start, end }
}

#[derive(PartialEq, Eq)]
struct Reindeer {
    score: usize,
    position: (i32, i32),
    direction: Direction,
    path: Vec<(i32, i32)>,
}

impl Ord for Reindeer {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.score.cmp(&other.score)
    }
}

impl PartialOrd for Reindeer {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

fn solve(input: &str) -> (usize, usize) {
    let Game { grid, start, end } = parse(input);
    let mut frontier = BinaryHeap::from([Reverse(Reindeer {
        score: 0,
        position: start,
        direction: Direction::East,
        path: vec![],
    })]);
    let mut scores = HashMap::from([((start, Direction::East), 0)]);
    let mut lowest_score = usize::MAX;
    let mut visited_best: HashSet<(i32, i32)> = HashSet::new();

    while let Some(Reverse(Reindeer {
        score,
        position,
        direction,
        mut path,
    })) = frontier.pop()
    {
        if position == end {
            if score <= lowest_score {
                lowest_score = score;
                visited_best.extend(path.iter());
            }
            continue;
        }

        for (next_position, next_direction) in next(&grid, position, direction) {
            let next_score = if direction == next_direction {
                score + 1
            } else {
                score + 1001
            };
            let score = scores.get(&(next_position, next_direction));
            if score.is_none() || score.is_some_and(|&score| next_score <= score) {
                scores.insert((next_position, next_direction), next_score);

                path.push(position);
                frontier.push(Reverse(Reindeer {
                    score: next_score,
                    position: next_position,
                    direction: next_direction,
                    path: path.clone(),
                }));
            }
        }
    }

    (lowest_score, visited_best.len() + 1)
}

fn main() {
    let (part1, part2) = solve(include_str!("../input.txt"));
    println!("part1={}", part1);
    println!("part2={}", part2);
}

#[cfg(test)]
mod test {
    use super::*;
    const INPUT: &str = "###############
#.......#....E#
#.#.###.#.###.#
#.....#.#...#.#
#.###.#####.#.#
#.#.#.......#.#
#.#.#####.###.#
#...........#.#
###.#.#####.#.#
#...#.....#.#.#
#.#.#.###.#.#.#
#.....#...#.#.#
#.###.#.#.#.#.#
#S..#.....#...#
###############";

    #[test]
    fn test_part1() {
        assert_eq!(solve(INPUT).0, 7036);
    }

    #[test]
    fn test_part2() {
        assert_eq!(solve(INPUT).1, 45);
    }
}
