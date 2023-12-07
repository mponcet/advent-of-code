use std::cmp::Ordering;
use std::collections::HashMap;

struct Hand(Vec<usize>);

fn parse(input: &str, with_joker: bool) -> Vec<(Hand, usize)> {
    fn card_score(c: u8, with_joker: bool) -> usize {
        match c {
            b'A' => 14,
            b'K' => 13,
            b'Q' => 12,
            b'J' => {
                if with_joker {
                    1
                } else {
                    11
                }
            }
            b'T' => 10,
            n => (n - b'0') as usize,
        }
    }

    input
        .lines()
        .filter_map(|l| {
            l.split_once(' ').map(|(hand, bid)| {
                (
                    Hand(hand.bytes().map(|c| card_score(c, with_joker)).collect()),
                    bid.parse().unwrap(),
                )
            })
        })
        .collect()
}

fn cmp_hands(hand_l: &Hand, hand_r: &Hand, with_joker: bool) -> std::cmp::Ordering {
    fn rank(hand: &Hand, with_joker: bool) -> usize {
        let mut nr_jokers = 0;
        let mut hand_counter_map = hand.0.iter().fold(HashMap::new(), |mut map, c| {
            *map.entry(c).or_insert(0) += 1;
            map
        });

        if with_joker {
            if let Some(count) = hand_counter_map.remove(&1) {
                if count == 5 {
                    return 7;
                }
                nr_jokers = count;
            }
        }

        let mut hand_counter: Vec<usize> = hand_counter_map.values().copied().collect();
        hand_counter.sort_unstable();
        *hand_counter.last_mut().unwrap() += nr_jokers;

        match &hand_counter[..] {
            [5] => 7,
            [1, 4] => 6,
            [2, 3] => 5,
            [1, 1, 3] => 4,
            [1, 2, 2] => 3,
            [1, 1, 1, 2] => 2,
            [1, 1, 1, 1, 1] => 1,
            _ => panic!("wtf"),
        }
    }

    match rank(hand_l, with_joker).cmp(&rank(hand_r, with_joker)) {
        Ordering::Greater => Ordering::Greater,
        Ordering::Less => Ordering::Less,
        Ordering::Equal => hand_l.0.cmp(&hand_r.0),
    }
}

fn calc_winnings(input: &str, with_joker: bool) -> usize {
    let mut hands = parse(input, with_joker);
    hands.sort_by(|h1, h2| cmp_hands(&h1.0, &h2.0, with_joker));

    hands
        .iter()
        .enumerate()
        .map(|(idx, (_, bid))| (idx + 1) * bid)
        .sum()
}

fn part1(input: &str) -> usize {
    calc_winnings(input, false)
}

fn part2(input: &str) -> usize {
    calc_winnings(input, true)
}

fn main() {
    println!("part1={}", part1(include_str!("../input.txt")));
    println!("part2={}", part2(include_str!("../input.txt")));
}

#[cfg(test)]
mod tests {
    use super::*;
    const TEST_INPUT: &str = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";

    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST_INPUT), 6440);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(TEST_INPUT), 5905);
    }
}
