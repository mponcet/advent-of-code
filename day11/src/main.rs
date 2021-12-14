const GRID_SIZE: usize = 10;

fn parse_input() -> Vec<Vec<u8>> {
    include_str!("../input.txt")
        .lines()
        .map(|l| l.bytes().map(|b| b - b'0').collect())
        .collect()
}

fn neighbors(i: usize, j: usize) -> impl Iterator<Item = (usize, usize)> {
    [
        (i.wrapping_sub(1), j.wrapping_sub(1)),
        (i.wrapping_sub(1), j),
        (i.wrapping_sub(1), j + 1),
        (i, j.wrapping_sub(1)),
        (i, j + 1),
        (i + 1, j.wrapping_sub(1)),
        (i + 1, j),
        (i + 1, j + 1),
    ]
    .into_iter()
    .filter(|&(i, j)| i < GRID_SIZE && j < GRID_SIZE)
}

fn flash(grid: &mut [Vec<u8>], i: usize, j: usize) -> usize {
    let mut count = 1;

    grid[i][j] = 0;
    for (ni, nj) in neighbors(i, j) {
        if grid[ni][nj] > 0 {
            grid[ni][nj] += 1;
            if grid[ni][nj] >= 10 {
                count += flash(grid, ni, nj);
            }
        }
    }

    count
}

fn octopuses_step(grid: &mut [Vec<u8>]) -> usize {
    let mut count = 0;

    grid.iter_mut().flatten().for_each(|p| *p += 1);

    for i in 0..GRID_SIZE {
        for j in 0..GRID_SIZE {
            if grid[i][j] >= 10 {
                count += flash(grid, i, j);
            }
        }
    }

    count
}

fn part1() -> usize {
    let mut grid = parse_input();
    (0..100).map(|_| octopuses_step(&mut grid)).sum()
}

fn part2() -> usize {
    let mut grid = parse_input();
    (0..)
        .find(|_| octopuses_step(&mut grid) == GRID_SIZE * GRID_SIZE)
        .map(|step| step + 1)
        .expect("Couldn't find step")
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
