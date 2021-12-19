use std::cmp::Reverse;
use std::collections::BinaryHeap;

const SCALE_FACTOR: usize = 5;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

struct Grid {
    grid: Vec<Vec<u8>>,
    size: usize,
}

impl Grid {
    fn parse_input(scale: bool) -> Result<Grid> {
        let grid: Vec<Vec<u8>> = include_str!("../input.txt")
            .lines()
            .map(|l| {
                l.chars()
                    .filter_map(|d| match d.to_digit(10) {
                        Some(d) => Some(d as u8),
                        _ => None,
                    })
                    .collect()
            })
            .collect();

        let grid_size = grid.len();
        if grid.iter().any(|g| grid_size != g.len()) {
            Err("corrupted map".into())
        } else if scale {
            let mut scaled_grid = vec![vec![0; grid_size * SCALE_FACTOR]; grid_size * SCALE_FACTOR];
            for x in 0..(grid_size * SCALE_FACTOR) {
                for y in 0..(grid_size * SCALE_FACTOR) {
                    scaled_grid[x][y] = (((grid[x % grid_size][y % grid_size]
                        + (x / grid_size + y / grid_size) as u8)
                        - 1)
                        % 9)
                        + 1;
                }
            }
            Ok(Grid {
                grid: scaled_grid,
                size: grid_size * SCALE_FACTOR,
            })
        } else {
            Ok(Grid {
                grid,
                size: grid_size,
            })
        }
    }

    fn neighbors<'a>(&self, (x, y): (usize, usize)) -> impl Iterator<Item = (usize, usize)> + '_ {
        [
            (Some(x), y.checked_add(1)),
            (Some(x), y.checked_sub(1)),
            (x.checked_add(1), Some(y)),
            (x.checked_sub(1), Some(y)),
        ]
        .into_iter()
        .filter_map(|(x, y)| match (x, y) {
            (Some(x), Some(y)) => Some((x, y)),
            _ => None,
        })
        .filter(|&(x, y)| x < self.size && y < self.size)
    }

    fn cost(&self, (x, y): (usize, usize)) -> u32 {
        self.grid[x][y] as u32
    }

    fn lowest_risk(&self) -> u32 {
        let mut visited = vec![vec![false; self.size]; self.size];
        let mut list = BinaryHeap::from([(Reverse(0), (0, 0))]);

        while let Some((Reverse(risk), curr)) = list.pop() {
            if curr == (self.size - 1, self.size - 1) {
                return risk;
            }

            for (nx, ny) in self.neighbors(curr) {
                if !visited[nx][ny] {
                    list.push((Reverse(risk + self.cost((nx, ny))), (nx, ny)));
                }
                visited[nx][ny] = true;
            }
        }

        0
    }
}

fn part1() -> Result<u32> {
    let grid = Grid::parse_input(false)?;
    Ok(grid.lowest_risk())
}

fn part2() -> Result<u32> {
    let grid = Grid::parse_input(true)?;
    Ok(grid.lowest_risk())
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        println!("answer: {}", part1().unwrap());
    }

    #[test]
    fn test_part2() {
        println!("answer: {}", part2().unwrap());
    }
}
