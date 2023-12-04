use std::collections::HashSet;

use regex::Regex;

#[derive(Debug)]
struct Card {
    winning_nums: Vec<usize>,
    nums: Vec<usize>,
}

fn parse(input: &str) -> Vec<Card> {
    let re = Regex::new(r"((?:\d+\s*)+)\|\s*((?:\d+\s*)+)$").unwrap();

    input
        .lines()
        .map(|l| {
            let caps = re.captures(l).unwrap();
            Card {
                winning_nums: caps
                    .get(1)
                    .unwrap()
                    .as_str()
                    .split_whitespace()
                    .map(|n| n.parse().unwrap())
                    .collect(),
                nums: caps
                    .get(2)
                    .unwrap()
                    .as_str()
                    .split_whitespace()
                    .map(|n| n.parse().unwrap())
                    .collect(),
            }
        })
        .collect()
}

fn part1(input: &str) -> usize {
    parse(input)
        .into_iter()
        .map(|card| {
            let winning: HashSet<_> = card.winning_nums.into_iter().collect();
            let nr_winning = card
                .nums
                .into_iter()
                .filter(|n| winning.contains(n))
                .count();
            if nr_winning == 0 {
                0
            } else {
                2usize.pow(nr_winning as u32 - 1)
            }
        })
        .sum()
}

fn part2(input: &str) -> usize {
    let cards = parse(input);
    let mut copies = vec![1usize; cards.len()];

    for (idx, card) in cards.iter().enumerate() {
        let winning: HashSet<_> = card.winning_nums.iter().collect();
        let nr_winning = card.nums.iter().filter(|n| winning.contains(n)).count();
        let nr_copies = copies[idx];

        for i in idx + 1..(idx + 1 + nr_winning).min(copies.len()) {
            copies[i] += nr_copies;
        }
    }

    copies.iter().sum::<usize>()
}

fn main() {
    println!("part1={}", part1(include_str!("../input.txt")));
    println!("part2={}", part2(include_str!("../input.txt")));
}

#[cfg(test)]
mod tests {
    use super::*;
    const TEST_INPUT: &str = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
    Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
    Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
    Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
    Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
    Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";

    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST_INPUT), 13);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(TEST_INPUT), 30);
    }
}
