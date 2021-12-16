use std::collections::HashSet;
use std::hash::{Hash, Hasher};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

#[derive(Debug, Copy, Clone)]
enum Fold {
    X(usize),
    Y(usize),
}

#[derive(Debug, PartialEq, Eq)]
struct Dot {
    x: usize,
    y: usize,
}

impl Hash for Dot {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.x.hash(state);
        self.y.hash(state);
    }
}

fn do_fold(dots: &mut [Dot], fold: Fold) {
    for dot in dots {
        match fold {
            Fold::X(x) => dot.x = if dot.x < x { dot.x } else { x + x - dot.x },
            Fold::Y(y) => dot.y = if dot.y < y { dot.y } else { y + y - dot.y },
        }
    }
}

fn parse_input() -> Result<(Vec<Dot>, Vec<Fold>)> {
    let (dots, folds) = include_str!("../input.txt")
        .split_once("\n\n")
        .ok_or("missing instructions")?;

    let dots = dots
        .lines()
        .filter_map(|l| {
            l.split_once(',').map(|(x, y)| {
                Ok(Dot {
                    x: x.parse()?,
                    y: y.parse()?,
                })
            })
        })
        .collect::<Result<_>>()?;

    let folds = folds
        .lines()
        .filter_map(|l| {
            l.split_once('=').map(|(xy, pos)| match xy.chars().last() {
                Some(c) => {
                    if c == 'x' {
                        Ok(Fold::X(pos.parse()?))
                    } else if c == 'y' {
                        Ok(Fold::Y(pos.parse()?))
                    } else {
                        Err("missing fold along instruction".into())
                    }
                }
                _ => Err("missing instruction".into()),
            })
        })
        .collect::<Result<_>>()?;

    Ok((dots, folds))
}

fn part1() -> Result<usize> {
    let (mut dots, folds) = parse_input()?;

    do_fold(
        &mut dots,
        folds
            .first()
            .ok_or("missing first fold instruction")?
            .clone(),
    );

    Ok(dots.into_iter().collect::<HashSet<_>>().len())
}

fn part2() -> Result<usize> {
    let (mut dots, folds) = parse_input()?;

    for fold in folds {
        do_fold(&mut dots, fold);
    }

    let (width, height) = dots.iter().fold((0, 0), |(width, height), dot| {
        (dot.x.max(width), dot.y.max(height))
    });
    let mut code = vec![vec!['.'; width + 1]; height + 1];
    for dot in dots {
        code[dot.y][dot.x] = '#';
    }
    for y in 0..=height {
        for x in 0..=width {
            print!("{}", code[y][x]);
        }
        println!("");
    }

    Ok(0)
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
