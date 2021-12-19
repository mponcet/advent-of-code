// optimal solution : https://www.reddit.com/r/adventofcode/comments/rar7ty/comment/hnk6gz0/
fn parse_input() -> Vec<i32> {
    include_str!("../input.txt")
        .split(',')
        .map(|x| x.trim().parse::<i32>().unwrap())
        .collect()
}

fn fuel_cost(positions: &[i32], position: i32, cost_fn: fn(i32, i32) -> i32) -> i32 {
    positions.iter().map(|&p| cost_fn(p, position)).sum()
}

fn part1() -> i32 {
    let mut positions = parse_input();

    let cost_fn = |x: i32, y: i32| (x - y).abs();
    let len = positions.len();
    // median
    let (_, &mut best_position, _) = positions.select_nth_unstable(len / 2);
    fuel_cost(&mut positions, best_position, cost_fn)
}

fn part2() -> i32 {
    let mut positions = parse_input();

    let cost_fn = |x: i32, y: i32| (x - y).abs() * ((x - y).abs() + 1) / 2;
    // mean: luck
    let best_position = positions.iter().sum::<i32>() / positions.len() as i32;
    fuel_cost(&mut positions, best_position, cost_fn)
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
