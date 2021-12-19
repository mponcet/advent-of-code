use regex::Regex;
use std::collections::HashMap;
use std::hash::{Hash, Hasher};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct Point {
    x: i32,
    y: i32,
}

impl Hash for Point {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.x.hash(state);
        self.y.hash(state);
    }
}

#[derive(Debug, Clone, Copy)]
struct Line {
    a: Point,
    b: Point,
}

struct LineIter {
    curr: Option<Point>,
    end: Point,
    dx: i32,
    dy: i32,
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

    fn points(self) -> LineIter {
        LineIter {
            curr: Some(self.a),
            end: self.b,
            dx: (self.b.x - self.a.x).signum(),
            dy: (self.b.y - self.a.y).signum(),
        }
    }
}

impl Iterator for LineIter {
    type Item = Point;

    fn next(&mut self) -> Option<Self::Item> {
        let curr = self.curr?;

        if curr == self.end {
            self.curr.take()
        } else {
            self.curr.replace(Point {
                x: curr.x + self.dx,
                y: curr.y + self.dy,
            })
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
            for p in line.points() {
                let counter = map.entry(p).or_insert(0);
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
