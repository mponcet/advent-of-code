fn parse(input: &str) -> Vec<Vec<isize>> {
    input
        .lines()
        .map(|l| l.split_whitespace().map(|n| n.parse().unwrap()).collect())
        .collect()
}

fn predict_part1(values: &[isize]) -> isize {
    let diffs: Vec<_> = values.windows(2).map(|w| w[1] - w[0]).collect();
    if diffs.iter().any(|&v| v != 0) {
        values.last().unwrap() + predict_part1(&diffs)
    } else {
        *values.last().unwrap()
    }
}

fn part1(input: &str) -> isize {
    parse(input)
        .iter()
        .map(|values| predict_part1(values))
        .sum()
}

fn predict_part2(values: &[isize]) -> isize {
    let diffs: Vec<_> = values.windows(2).map(|w| w[1] - w[0]).collect();
    if diffs.iter().any(|&v| v != 0) {
        values.first().unwrap() - predict_part2(&diffs)
    } else {
        *values.first().unwrap()
    }
}

fn part2(input: &str) -> isize {
    parse(input)
        .iter()
        .map(|values| predict_part2(values))
        .sum()
}

fn main() {
    println!("part1={}", part1(include_str!("../input.txt")));
    println!("part2={}", part2(include_str!("../input.txt")));
}

#[cfg(test)]
mod tests {
    use super::*;
    const TEST_INPUT: &str = "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45";

    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST_INPUT), 114);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(TEST_INPUT), 2);
    }
}
