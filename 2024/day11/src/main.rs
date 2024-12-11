use std::collections::HashMap;

fn parse(input: &str) -> HashMap<usize, usize> {
    input.split_whitespace().fold(HashMap::new(), |mut h, n| {
        let n = n.parse().unwrap();
        h.entry(n).and_modify(|e| *e += 1).or_insert(1);
        h
    })
}

fn is_number_of_digits_even(n: usize) -> bool {
    (n.checked_ilog10().unwrap_or(0) + 1) % 2 == 0
}

fn blink(stones: HashMap<usize, usize>) -> HashMap<usize, usize> {
    let mut new_stones = HashMap::new();

    for (stone, count) in stones {
        if stone == 0 {
            new_stones
                .entry(1)
                .and_modify(|e| *e += count)
                .or_insert(count);
        } else if is_number_of_digits_even(stone) {
            let new_stone = stone.to_string();
            let (new_stone1, new_stone2) = (
                new_stone[0..new_stone.len() / 2].parse().unwrap(),
                new_stone[new_stone.len() / 2..].parse().unwrap(),
            );
            new_stones
                .entry(new_stone1)
                .and_modify(|e| *e += count)
                .or_insert(count);
            new_stones
                .entry(new_stone2)
                .and_modify(|e| *e += count)
                .or_insert(count);
        } else {
            new_stones
                .entry(stone * 2024)
                .and_modify(|e| *e += count)
                .or_insert(count);
        }
    }

    new_stones
}

fn part1(input: &str) -> usize {
    let mut stones = parse(input);

    for _ in 0..25 {
        stones = blink(stones);
    }

    stones.values().sum()
}

fn part2(input: &str) -> usize {
    let mut stones = parse(input);

    for _ in 0..75 {
        stones = blink(stones);
    }

    stones.values().sum()
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
        let input = "125 17";
        assert_eq!(part1(input), 55312);
    }
}
