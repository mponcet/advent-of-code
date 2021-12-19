#[derive(Clone, Copy, Debug, Default)]
struct BingoNumber {
    number: u32,
    checked: bool,
}

#[derive(Clone, Copy, Debug, Default)]
struct BingoBoard {
    grid: [[BingoNumber; 5]; 5],
    won: bool,
}

impl BingoBoard {
    fn check_number(&mut self, number: u32) {
        self.grid
            .iter_mut()
            .flatten()
            .filter(|n| n.number == number)
            .for_each(|n| n.checked = true);
    }

    fn wins(&self) -> bool {
        let row_bingo = |grid: &[[BingoNumber; 5]; 5]| -> bool {
            grid.iter().any(|row| row.iter().all(|n| n.checked == true))
        };
        let column_bingo = |grid: &[[BingoNumber; 5]; 5]| -> bool {
            (0..5).any(|col| grid[..][col].iter().all(|n| n.checked == true))
        };
        row_bingo(&self.grid) || column_bingo(&self.grid)
    }

    fn score(&self) -> u32 {
        self.grid
            .iter()
            .flatten()
            .filter(|n| n.checked == false)
            .map(|n| n.number)
            .sum()
    }
}

fn parse_input(input: &str) -> (Vec<u32>, Vec<BingoBoard>) {
    let mut lines = input.split("\n\n");
    let winning_numbers: Vec<_> = lines
        .next()
        .unwrap()
        .split(",")
        .map(|n| n.parse::<u32>().unwrap())
        .collect();

    let boards = lines
        .map(|l| {
            let mut grid: [[BingoNumber; 5]; 5] = Default::default();

            l.split_whitespace().enumerate().for_each(|(idx, val)| {
                grid[idx / 5][idx % 5] = BingoNumber {
                    number: val.parse::<u32>().unwrap(),
                    checked: false,
                }
            });

            BingoBoard { grid, won: false }
        })
        .collect();

    (winning_numbers, boards)
}
fn part1() -> u32 {
    let input = include_str!("../input.txt");
    let (winning_numbers, mut boards) = parse_input(&input);

    for wn in winning_numbers {
        for board in boards.iter_mut() {
            board.check_number(wn);

            if board.wins() {
                return wn * board.score();
            }
        }
    }

    panic!("no winning board");
}

fn part2() -> u32 {
    let input = include_str!("../input.txt");
    let (winning_numbers, mut boards) = parse_input(&input);

    for wn in winning_numbers {
        for board in boards.iter_mut() {
            board.check_number(wn);

            if board.wins() {
                board.won = true;
            }
        }
        if boards.len() == 1 && boards.first().unwrap().won == true {
            return wn * boards.first().unwrap().score();
        } else {
            boards.retain(|b| b.won == false);
        }
    }

    panic!("no winning board");
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
