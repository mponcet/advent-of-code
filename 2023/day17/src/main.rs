use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap},
};

#[derive(Debug)]
struct Grid {
    grid: Vec<usize>,
    columns: usize,
    rows: usize,
}

impl Grid {
    fn get(&self, (i, j): (usize, usize)) -> Option<usize> {
        if i < self.columns && j < self.rows {
            self.grid.get(j * self.columns + i).copied()
        } else {
            None
        }
    }

    fn neighbors(
        &self,
        (i, j): (usize, usize),
        direction: Direction,
    ) -> impl Iterator<Item = ((usize, usize), Direction)> + '_ {
        [
            ((i + 1, j), Direction::Right),
            ((i.wrapping_sub(1), j), Direction::Left),
            ((i, j + 1), Direction::Up),
            ((i, j.wrapping_sub(1)), Direction::Down),
        ]
        .into_iter()
        .filter(move |&((x, y), d)| x < self.columns && y < self.rows && d != direction.flip())
    }
}

fn parse(input: &str) -> Grid {
    let mut columns = 0;
    let grid: Vec<_> = input
        .lines()
        .flat_map(|l| {
            columns = l.len();
            l.bytes().map(|b| (b - b'0') as usize)
        })
        .collect();

    let rows = grid.len() / columns;

    Grid {
        grid,
        columns,
        rows,
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
enum Direction {
    Right,
    Left,
    Up,
    Down,
}

impl Direction {
    fn flip(&self) -> Self {
        match self {
            Direction::Right => Direction::Left,
            Direction::Left => Direction::Right,
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
struct State {
    cost: usize,
    pos: (usize, usize),
    direction: Direction,
    steps: usize,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.cost
            .cmp(&other.cost)
            .then_with(|| self.pos.cmp(&other.pos))
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

//https://www.reddit.com/r/adventofcode/comments/18k9ne5/comment/kdsgnvn
#[derive(Debug, PartialEq, Eq, Hash)]
struct CostKey {
    pos: (usize, usize),
    direction: Direction,
    steps: usize,
}

impl From<State> for CostKey {
    fn from(s: State) -> Self {
        Self {
            pos: s.pos,
            direction: s.direction,
            steps: s.steps,
        }
    }
}

fn minimize_heat_loss(
    grid: &Grid,
    start: (usize, usize),
    goal: (usize, usize),
    min_steps: usize,
    max_steps: usize,
) -> usize {
    let s1 = State {
        cost: 0,
        pos: start,
        direction: Direction::Right,
        steps: 0,
    };
    let s2 = State {
        cost: 0,
        pos: start,
        direction: Direction::Up,
        steps: 0,
    };
    let mut costs: HashMap<CostKey, usize> = HashMap::from([(s1.into(), 0), (s2.into(), 0)]);
    let mut frontier = BinaryHeap::from([Reverse(s1), Reverse(s2)]);

    while let Some(Reverse(current)) = frontier.pop() {
        if current.pos == goal && current.steps >= min_steps {
            return current.cost;
        }
        if costs
            .get(&current.into())
            .is_some_and(|&c| c < current.cost)
        {
            continue;
        }

        for (n, d) in grid.neighbors(current.pos, current.direction) {
            let next = State {
                cost: current.cost + grid.get(n).unwrap(),
                pos: n,
                direction: d,
                steps: if d == current.direction {
                    current.steps + 1
                } else {
                    1
                },
            };

            if next.steps > max_steps {
                continue;
            }

            if next.direction != current.direction && current.steps < min_steps {
                continue;
            }

            if !costs.contains_key(&next.into()) || (next.cost < *costs.get(&next.into()).unwrap())
            {
                frontier.push(Reverse(next));
                costs.insert(next.into(), next.cost);
            }
        }
    }

    0
}

fn part1(input: &str) -> usize {
    let grid = parse(input);
    minimize_heat_loss(&grid, (0, 0), (grid.columns - 1, grid.rows - 1), 1, 3)
}

fn part2(input: &str) -> usize {
    let grid = parse(input);
    minimize_heat_loss(&grid, (0, 0), (grid.columns - 1, grid.rows - 1), 4, 10)
}

fn main() {
    println!("part1={}", part1(include_str!("../input.txt")));
    println!("part2={}", part2(include_str!("../input.txt")));
}

#[cfg(test)]
mod tests {
    use super::*;
    const TEST_INPUT: &str = "2413432311323
3215453535623
3255245654254
3446585845452
4546657867536
1438598798454
4457876987766
3637877979653
4654967986887
4564679986453
1224686865563
2546548887735
4322674655533";

    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST_INPUT), 102);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(TEST_INPUT), 94);
    }

    #[test]
    fn test_part2_2() {
        let input = "111111111111
999999999991
999999999991
999999999991
999999999991";
        assert_eq!(part2(input), 71);
    }
}
