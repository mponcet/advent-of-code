use itertools::Itertools;

fn parse(input: &str) -> Vec<Vec<i32>> {
    input
        .lines()
        .map(|report| {
            report
                .split(' ')
                .map(|level| level.parse().unwrap())
                .collect()
        })
        .collect()
}

fn is_safe_report(report: &[i32], skip_at: Option<usize>) -> bool {
    report
        .iter()
        .enumerate()
        .filter(|(i, _)| skip_at.is_none_or(|skip_at| skip_at != *i))
        .map(|(_, v)| v)
        .tuple_windows()
        .map(|(a, b)| a - b)
        .all(|n| n.is_positive() && n <= 3)
        || report
            .iter()
            .enumerate()
            .filter(|(i, _)| skip_at.is_none_or(|skip_at| skip_at != *i))
            .map(|(_, v)| v)
            .tuple_windows()
            .map(|(a, b)| a - b)
            .all(|n| n.is_negative() && n >= -3)
}

fn part1(input: &str) -> u32 {
    let reports = parse(input);

    reports
        .iter()
        .map(|report| if is_safe_report(report, None) { 1 } else { 0 })
        .sum()
}

fn part2(input: &str) -> u32 {
    let reports = parse(input);

    reports
        .iter()
        .map(|report| {
            if is_safe_report(report, None)
                || (0..report.len()).any(|idx| is_safe_report(report, Some(idx)))
            {
                1
            } else {
                0
            }
        })
        .sum()
}

fn main() {
    println!("part1={}", part1(include_str!("../input.txt")));
    println!("part2={}", part2(include_str!("../input.txt")));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9";
        assert_eq!(part1(input), 2);
    }

    #[test]
    fn test_part2() {
        let input = "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9";
        assert_eq!(part2(input), 4);
    }
}
