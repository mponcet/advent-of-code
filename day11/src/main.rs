const STEPS: usize = 100;
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

fn flash(grid: &mut [Vec<u8>], i: usize, j: usize) -> u32 {
    let mut count = 0;

    for (ni, nj) in neighbors(i, j) {
        grid[ni][nj] += 1;
        if grid[ni][nj] == 10 {
            count += 1 + flash(grid, ni, nj);
        }
    }

    count
}

fn octopuses_run(steps: usize) -> u32 {
    let mut count = 0;
    let mut grid = parse_input();

    for step in 0..steps {
        if grid.iter().flatten().all(|p| *p == 0) {
            return step as u32;
        }

        grid.iter_mut().flatten().for_each(|p| *p += 1);

        let mut will_flash: Vec<(usize, usize)> = Vec::new();
        for i in 0..GRID_SIZE {
            for j in 0..GRID_SIZE {
                if grid[i][j] > 9 {
                    will_flash.push((i, j));
                    count += 1;
                }
            }
        }
        for (i, j) in will_flash {
            count += flash(&mut grid, i, j);
        }

        grid.iter_mut()
            .flatten()
            .filter(|p| **p > 9)
            .for_each(|p| *p = 0);
    }

    count
}

fn part1() -> u32 {
    octopuses_run(STEPS)
}

fn part2() -> u32 {
    octopuses_run(usize::MAX)
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
