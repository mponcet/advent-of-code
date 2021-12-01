fn part1() -> usize {
    let input = include_str!("../input.txt");
    let lines = input.lines().map(|l| l.parse::<u32>().unwrap());

    lines
        .clone()
        .zip(lines.skip(1))
        .filter(|(depth1, depth2)| depth2 > depth1)
        .count()
}

fn part2() -> usize {
    let input = include_str!("../input.txt");
    let lines: Vec<_> = input.lines().map(|l| l.parse::<u32>().unwrap()).collect();

    lines
        .windows(3)
        .zip(lines[1..].windows(3))
        .filter(|(w1, w2)| w2.into_iter().sum::<u32>() > w1.into_iter().sum::<u32>())
        .count()
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
