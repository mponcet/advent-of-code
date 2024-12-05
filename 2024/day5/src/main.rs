use std::{cmp::Ordering, collections::HashSet};

#[derive(Debug)]
struct Game {
    rules: HashSet<(usize, usize)>,
    updates: Vec<Vec<usize>>,
}

fn parse(input: &str) -> Game {
    let (rules, updates) = input.split_once("\n\n").unwrap();

    let rules = rules
        .lines()
        .map(|line| {
            let (before, after) = line.split_once('|').unwrap();
            (before.parse().unwrap(), after.parse().unwrap())
        })
        .collect();

    let updates = updates
        .lines()
        .map(|line| line.split(',').map(|page| page.parse().unwrap()).collect())
        .collect();

    Game { rules, updates }
}

fn part1(input: &str) -> usize {
    let game = parse(input);

    game.updates
        .iter()
        .filter_map(|update| {
            let correct = update.iter().enumerate().all(|(i, page)| {
                let pages_after = &update[i + 1..];
                pages_after
                    .iter()
                    .all(|page_after| game.rules.contains(&(*page, *page_after)))
            });

            if correct {
                Some(update[update.len() / 2])
            } else {
                None
            }
        })
        .sum()
}

fn part2(input: &str) -> usize {
    let game = parse(input);

    let mut incorrects = game
        .updates
        .iter()
        .filter(|update| {
            update.iter().enumerate().any(|(i, page)| {
                let pages_after = &update[i + 1..];
                pages_after
                    .iter()
                    .any(|page_after| !game.rules.contains(&(*page, *page_after)))
            })
        })
        .cloned()
        .collect::<Vec<_>>();

    incorrects
        .iter_mut()
        .map(|incorrect| {
            incorrect.sort_unstable_by(|a, b| {
                if game.rules.contains(&(*a, *b)) {
                    Ordering::Less
                } else {
                    Ordering::Equal
                }
            });

            incorrect[incorrect.len() / 2]
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

    const INPUT: &str = "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47";

    #[test]
    fn test_part1() {
        assert_eq!(part1(INPUT), 143);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(INPUT), 123);
    }
}
