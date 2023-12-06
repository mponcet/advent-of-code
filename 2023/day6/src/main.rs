fn parse_part1(input: &str) -> Vec<(f64, f64)> {
    let mut lines = input.lines();
    lines
        .next()
        .unwrap()
        .split_whitespace()
        .zip(lines.next().unwrap().split_whitespace())
        .skip(1)
        .map(|(time, distance)| (time.parse().unwrap(), distance.parse().unwrap()))
        .collect()
}

fn part1(input: &str) -> usize {
    parse_part1(input)
        .iter()
        .filter_map(|&(time, distance)| {
            let delta = time * time - 4f64 * distance;

            if delta <= 0.0 {
                None
            } else {
                let x1 = f64::floor((-time + f64::sqrt(delta)) / -2f64);
                let x2 = f64::ceil((-time - f64::sqrt(delta)) / -2f64);
                Some((x2 - x1 - 1.0) as usize)
            }
        })
        .product()
}

fn parse_part2(input: &str) -> (f64, f64) {
    let mut time_distance = input.lines().map(|l| {
        l.split_once(' ')
            .map(|(_, nums)| nums.replace(' ', "").parse::<f64>().unwrap())
            .unwrap()
    });

    (time_distance.next().unwrap(), time_distance.next().unwrap())
}

fn part2(input: &str) -> usize {
    let (time, distance) = parse_part2(input);

    let delta = time * time - 4f64 * distance;
    if delta <= 0.0 {
        0
    } else {
        let x1 = f64::floor((-time + f64::sqrt(delta)) / -2f64);
        let x2 = f64::ceil((-time - f64::sqrt(delta)) / -2f64);
        (x2 - x1 - 1.0) as usize
    }
}

fn main() {
    println!("part1={}", part1(include_str!("../input.txt")));
    println!("part2={}", part2(include_str!("../input.txt")));
}

#[cfg(test)]
mod tests {
    use super::*;
    const TEST_INPUT: &str = "Time:      7  15   30
Distance:  9  40  200";

    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST_INPUT), 288);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(TEST_INPUT), 71503);
    }
}
