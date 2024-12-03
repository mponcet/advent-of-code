use std::collections::HashMap;

fn parse(input: &str) -> (Vec<u32>, Vec<u32>) {
    input
        .lines()
        .filter_map(|line| {
            line.split_once("   ")
                .map(|(a, b)| (a.parse::<u32>().unwrap(), b.parse::<u32>().unwrap()))
        })
        .unzip()
}
fn part1(input: &str) -> u32 {
    let (mut v1, mut v2) = parse(input);

    v1.sort_unstable();
    v2.sort_unstable();

    v1.iter().zip(v2.iter()).map(|(a, b)| a.abs_diff(*b)).sum()
}

fn part2(input: &str) -> u32 {
    let (v1, v2) = parse(input);
    let mut count = HashMap::new();

    for n in v2.iter() {
        *count.entry(n).or_insert(0) += 1;
    }

    v1.iter().map(|a| a * count.get(a).unwrap_or(&0)).sum()
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
        let input = "3   4
4   3
2   5
1   3
3   9
3   3";
        assert_eq!(part1(input), 11);
    }

    #[test]
    fn test_part2() {
        let input = "3   4
4   3
2   5
1   3
3   9
3   3";
        assert_eq!(part2(input), 31);
    }
}
