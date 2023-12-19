#[derive(Clone, Copy, Debug)]
struct Point {
    x: isize,
    y: isize,
}
type Polygon = Vec<Point>;
type Perimeter = usize;

fn parse(input: &str, part2: bool) -> (Polygon, Perimeter) {
    fn next_point(current: Point, direction: char, distance: isize) -> Point {
        match direction {
            'R' => Point {
                x: current.x + distance,
                y: current.y,
            },
            'L' => Point {
                x: current.x - distance,
                y: current.y,
            },
            'U' => Point {
                x: current.x,
                y: current.y - distance,
            },
            'D' => Point {
                x: current.x,
                y: current.y + distance,
            },
            _ => unreachable!(),
        }
    }

    let mut p = Point { x: 0, y: 0 };
    let mut perimeter = 0;
    let polygon = input
        .lines()
        .map(|l| {
            let mut it = l.split_whitespace();
            match (it.next(), it.next(), it.next()) {
                (Some(direction), Some(distance), Some(rgb)) => {
                    if !part2 {
                        let direction = direction.chars().next().unwrap();
                        let distance = distance.parse::<isize>().unwrap();
                        perimeter += distance;
                        p = next_point(p, direction, distance);
                        p
                    } else {
                        let direction = match rgb.chars().nth(rgb.len() - 2).unwrap() {
                            '0' => 'R',
                            '1' => 'D',
                            '2' => 'L',
                            '3' => 'U',
                            _ => unreachable!(),
                        };
                        let distance = isize::from_str_radix(&rgb[2..rgb.len() - 2], 16).unwrap();
                        perimeter += distance;
                        p = next_point(p, direction, distance);
                        p
                    }
                }
                _ => unreachable!(),
            }
        })
        .collect::<Vec<_>>();

    (polygon, perimeter as usize)
}

fn inner_area(polygon: &Polygon) -> usize {
    let n = polygon.len() as isize;
    let area = (0..n - 1)
        .map(|i| {
            polygon[i as usize].x * polygon[(i + 1) as usize].y
                - polygon[i as usize].y * polygon[(i + 1) as usize].x
        })
        .sum::<isize>();

    area as usize
}

fn part1(input: &str) -> usize {
    let (polygons, perimeter) = parse(input, false);
    // shoelace + pick
    (inner_area(&polygons) + perimeter) / 2 + 1
}

fn part2(input: &str) -> usize {
    let (polygons, perimeter) = parse(input, true);
    // shoelace + pick
    (inner_area(&polygons) + perimeter) / 2 + 1
}

fn main() {
    println!("part1={}", part1(include_str!("../input.txt")));
    println!("part2={}", part2(include_str!("../input.txt")));
}

#[cfg(test)]
mod tests {
    use super::*;
    const TEST_INPUT: &str = "R 6 (#70c710)
D 5 (#0dc571)
L 2 (#5713f0)
D 2 (#d2c081)
R 2 (#59c680)
D 2 (#411b91)
L 5 (#8ceee2)
U 2 (#caa173)
L 1 (#1b58a2)
U 2 (#caa171)
R 2 (#7807d2)
U 3 (#a77fa3)
L 2 (#015232)
U 2 (#7a21e3)";

    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST_INPUT), 62);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(TEST_INPUT), 952408144115);
    }
}
