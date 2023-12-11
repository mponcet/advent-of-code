use std::collections::HashSet;

#[derive(Debug)]
struct Grid {
    tiles: Vec<u8>,
    max_i: usize,
    max_j: usize,
    start: (usize, usize),
}

enum Move {
    Left(usize, usize),
    Right(usize, usize),
    Up(usize, usize),
    Down(usize, usize),
}

impl Grid {
    fn get(&self, i: usize, j: usize) -> Option<u8> {
        if i < self.max_i && j < self.max_j {
            Some(self.tiles[j * self.max_i + i])
        } else {
            None
        }
    }

    fn set(&mut self, i: usize, j: usize, value: u8) {
        self.tiles[j * self.max_i + i] = value;
    }
}

fn parse(input: &str) -> Grid {
    let tiles: Vec<_> = input.lines().flat_map(|l| l.bytes()).collect();
    let max_i = input.lines().next().unwrap().len();
    let max_j = input.lines().count();
    let start = tiles
        .iter()
        .position(|&c| c == b'S')
        .map(|idx| (idx % max_i, idx / max_i))
        .unwrap();

    Grid {
        tiles,
        max_i,
        max_j,
        start,
    }
}
fn next_pipes(
    grid: &Grid,
    visited: &mut HashSet<(usize, usize)>,
    i: usize,
    j: usize,
) -> Vec<(usize, usize)> {
    [
        Move::Left(i.wrapping_sub(1), j),
        Move::Right(i + 1, j),
        Move::Up(i, j.wrapping_sub(1)),
        Move::Down(i, j + 1),
    ]
    .into_iter()
    .filter_map(|motion| match motion {
        Move::Left(ni, nj) => match grid.get(ni, nj) {
            Some(b'-') | Some(b'F') | Some(b'L') => Some((ni, nj)),
            _ => None,
        },
        Move::Right(ni, nj) => match grid.get(ni, nj) {
            Some(b'-') | Some(b'J') | Some(b'7') => Some((ni, nj)),
            _ => None,
        },
        Move::Up(ni, nj) => match grid.get(ni, nj) {
            Some(b'|') | Some(b'F') | Some(b'7') => Some((ni, nj)),
            _ => None,
        },
        Move::Down(ni, nj) => match grid.get(ni, nj) {
            Some(b'|') | Some(b'J') | Some(b'L') => Some((ni, nj)),
            _ => None,
        },
    })
    .filter(|ij| !visited.contains(ij))
    .collect()
}

fn loop_path(
    grid: &Grid,
    current: (usize, usize),
    visited: &mut HashSet<(usize, usize)>,
) -> Vec<(usize, usize)> {
    let mut current = current;
    let mut path = Vec::new();

    visited.insert(current);
    path.push(current);
    loop {
        let next_pipes = next_pipes(grid, visited, current.0, current.1);

        if next_pipes.is_empty() {
            break;
        } else if next_pipes.len() == 1 {
            current = next_pipes[0];
            visited.insert(current);
            path.push(current);
        } else {
            let max_path: Vec<_> = next_pipes
                .iter()
                .map(|&pipe| loop_path(grid, pipe, visited))
                .max_by(|path1, path2| path1.len().cmp(&path2.len()))
                .unwrap();

            path.extend_from_slice(&max_path);
        }
    }

    path
}

fn part1(input: &str) -> usize {
    let grid = parse(input);
    let mut visited: HashSet<(usize, usize)> = HashSet::new();
    let path = loop_path(&grid, grid.start, &mut visited);

    path.len() / 2
}

// https://en.wikipedia.org/wiki/Point_in_polygon
fn part2(input: &str, replace_start_with: u8) -> usize {
    let mut grid = parse(input);
    let mut visited: HashSet<(usize, usize)> = HashSet::new();
    let path = loop_path(&grid, grid.start, &mut visited);
    let path_set: HashSet<_> = path.iter().collect();

    grid.set(grid.start.0, grid.start.1, replace_start_with);

    let mut count = 0;
    for j in 0..grid.max_j {
        let mut i = 0;
        let mut in_loop = false;

        while i < grid.max_i {
            if path_set.contains(&(i, j)) {
                let c = grid.get(i, j).unwrap();

                if c == b'|' {
                    in_loop = !in_loop;
                } else {
                    i += 1;
                    while grid.get(i, j).unwrap() == b'-' {
                        i += 1;
                    }
                    let c_end = grid.get(i, j).unwrap();
                    // we crossed a line (in -> out or out -> in)
                    if (c, c_end) == (b'L', b'7') || (c, c_end) == (b'F', b'J') {
                        in_loop = !in_loop;
                    }
                }
                i += 1;
            } else if in_loop {
                while !path_set.contains(&(i, j)) {
                    count += 1;
                    i += 1;
                }
            } else {
                i += 1;
            }
        }
    }

    count
}

fn main() {
    println!("part1={}", part1(include_str!("../input.txt")));
    println!("part2={}", part2(include_str!("../input.txt"), b'|'));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = "..F7.
.FJ|.
SJ.L7
|F--J
LJ...";
        assert_eq!(part1(input), 8);
    }

    #[test]
    fn test_part2() {
        let input = ".F----7F7F7F7F-7....
.|F--7||||||||FJ....
.||.FJ||||||||L7....
FJL7L7LJLJ||LJ.L-7..
L--J.L7...LJS7F-7L7.
....F-J..F7FJ|L7L7L7
....L7.F7||L7|.L7L7|
.....|FJLJ|FJ|F7|.LJ
....FJL-7.||.||||...
....L---J.LJ.LJLJ...";
        assert_eq!(part2(input, b'F'), 8);
    }

    #[test]
    fn test_part2_2() {
        let input = "FF7FSF7F7F7F7F7F---7
L|LJ||||||||||||F--J
FL-7LJLJ||||||LJL-77
F--JF--7||LJLJ7F7FJ-
L---JF-JLJ.||-FJLJJ7
|F|F-JF---7F7-L7L|7|
|FFJF7L7F-JF7|JL---7
7-L-JL7||F7|L7F-7F7|
L.L7LFJ|||||FJL7||LJ
L7JLJL-JLJLJL--JLJ.L";
        assert_eq!(part2(input, b'7'), 10);
    }
}
