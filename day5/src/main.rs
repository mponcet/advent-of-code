use regex::Regex;
use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Debug, Clone, Copy)]
struct Line {
    a: Point,
    b: Point,
}

impl Line {
    fn is_horizontal(&self) -> bool {
        self.a.x == self.b.x
    }

    fn is_vertical(&self) -> bool {
        self.a.y == self.b.y
    }

    fn is_diagonal(&self) -> bool {
        (self.a.x - self.b.x).abs() == (self.a.y - self.b.y).abs()
    }
}

struct LineIter {
    curr: Option<Point>,
    last: Point,
    dx: i32,
    dy: i32,
}

impl IntoIterator for Line {
    type Item = Point;
    type IntoIter = LineIter;

    fn into_iter(self) -> Self::IntoIter {
        let dx = (self.a.x - self.b.x)
            .abs()
            .checked_div(self.b.x - self.a.x)
            .unwrap_or(0);

        let dy = (self.a.y - self.b.y)
            .abs()
            .checked_div(self.b.y - self.a.y)
            .unwrap_or(0);

        LineIter {
            curr: Some(self.a),
            last: self.b,
            dx,
            dy,
        }
    }
}

impl Iterator for LineIter {
    type Item = Point;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(curr) = self.curr {
            if curr == self.last {
                self.curr.take()
            } else {
                self.curr = Some(Point {
                    x: curr.x + self.dx,
                    y: curr.y + self.dy,
                });
                Some(curr)
            }
        } else {
            None
        }
    }
}

fn parse_input(input: &str) -> Vec<Line> {
    let re = Regex::new(r"(\d+)").unwrap();

    input
        .lines()
        .flat_map(|l| re.find_iter(l).map(|x| x.as_str().parse::<i32>().unwrap()))
        .collect::<Vec<_>>()
        .chunks(4)
        .map(|c| Line {
            a: Point { x: c[0], y: c[1] },
            b: Point { x: c[2], y: c[3] },
        })
        .collect()
}

fn part1(filter_diagonal: bool) -> u32 {
    let input = include_str!("../input.txt");
    let lines = parse_input(input);

    let mut map = HashMap::new();
    let mut overlapping_counter = 0;

    for line in lines {
        if line.is_horizontal() || line.is_vertical() || (filter_diagonal && line.is_diagonal()) {
            for p in line {
                let counter = map.entry((p.x, p.y)).or_insert(0);
                *counter += 1;

                if *counter == 2 {
                    overlapping_counter += 1;
                }
            }
        }
    }

    overlapping_counter
}

fn part2() -> u32 {
    part1(true)
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        println!("answer: {}", part1(false));
    }

    #[test]
    fn test_part2() {
        println!("answer: {}", part2());
    }
}
