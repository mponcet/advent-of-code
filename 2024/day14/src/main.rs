use std::{cmp::Ordering, collections::HashMap};

#[derive(Debug)]
struct Robot {
    row: i32,
    col: i32,
    velocity_row: i32,
    velocity_col: i32,
}

fn parse(input: &str) -> Vec<Robot> {
    input
        .lines()
        .filter_map(|line| {
            line.split_once(' ').map(|(pos, velocity)| {
                let (col, row) = pos[2..].split_once(',').unwrap();
                let (v_col, v_row) = velocity[2..].split_once(',').unwrap();
                Robot {
                    row: row.parse().unwrap(),
                    col: col.parse().unwrap(),
                    velocity_row: v_row.parse().unwrap(),
                    velocity_col: v_col.parse().unwrap(),
                }
            })
        })
        .collect()
}

fn wrap_index(idx: i32, max: i32) -> i32 {
    if idx >= max {
        idx % max
    } else if idx < 0 {
        let wrapped = idx % max;
        if wrapped == 0 {
            0
        } else {
            max + wrapped
        }
    } else {
        idx
    }
}

fn count(robots: &[Robot], rows: i32, cols: i32) -> usize {
    let (mut top_left, mut top_right, mut bottom_right, mut bottom_left) = (0, 0, 0, 0);

    for robot in robots {
        match robot.row.cmp(&(rows / 2)) {
            Ordering::Less if robot.col < cols / 2 => {
                top_left += 1;
            }
            Ordering::Less if robot.col > cols / 2 => {
                top_right += 1;
            }
            Ordering::Less => {}
            Ordering::Greater if robot.col < cols / 2 => {
                bottom_left += 1;
            }
            Ordering::Greater if robot.col > cols / 2 => {
                bottom_right += 1;
            }
            _ => {}
        }
    }

    top_right * top_left * bottom_left * bottom_right
}

fn print(robots: &[Robot], rows: i32, cols: i32) {
    let map = robots.iter().fold(
        HashMap::new(),
        |mut map: HashMap<(i32, i32), usize>, robot| {
            map.entry((robot.row, robot.col))
                .and_modify(|e| *e += 1)
                .or_insert(1);
            map
        },
    );

    for row in 0..rows {
        for col in 0..cols {
            if map.contains_key(&(row, col)) {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!();
    }
}

fn part1(input: &str, rows: i32, cols: i32) -> usize {
    let mut robots = parse(input);
    for _ in 0..100 {
        for robot in robots.iter_mut() {
            robot.row = wrap_index(robot.row + robot.velocity_row, rows);
            robot.col = wrap_index(robot.col + robot.velocity_col, cols);
        }
    }

    count(&robots, rows, cols)
}

fn part2(input: &str, rows: i32, cols: i32) {
    let mut robots = parse(input);
    for step in 1..20000 {
        for robot in robots.iter_mut() {
            robot.row = wrap_index(robot.row + robot.velocity_row, rows);
            robot.col = wrap_index(robot.col + robot.velocity_col, cols);
        }

        println!("=========> step {step}");
        print(&robots, rows, cols);
    }
}

fn main() {
    println!("part1={}", part1(include_str!("../input.txt"), 103, 101));
    part2(include_str!("../input.txt"), 103, 101);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = "p=0,4 v=3,-3
p=6,3 v=-1,-3
p=10,3 v=-1,2
p=2,0 v=2,-1
p=0,0 v=1,3
p=3,0 v=-2,-2
p=7,6 v=-1,-3
p=3,0 v=-1,-2
p=9,3 v=2,3
p=7,3 v=-1,2
p=2,4 v=2,-3
p=9,5 v=-3,-3";
        assert_eq!(part1(input, 7, 11), 12);
    }
}
