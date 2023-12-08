use std::collections::HashMap;

#[derive(Debug)]
struct Game {
    instrs: &'static str,
    net: HashMap<&'static str, (&'static str, &'static str)>,
}

fn parse(input: &'static str) -> Game {
    let mut lines = input.lines();

    let instrs = lines.next().unwrap();
    let net = lines
        .skip(1)
        .map(|l| (&l[0..3], &l[7..10], &l[12..15]))
        .fold(HashMap::new(), |mut map, l| {
            map.insert(l.0, (l.1, l.2));
            map
        });

    Game { instrs, net }
}

fn part1(input: &'static str) -> usize {
    let game = parse(input);
    let mut steps = 0;
    let mut current = "AAA";
    let mut instrs = game.instrs.chars().cycle();

    loop {
        current = game
            .net
            .get(current)
            .map(|&(left, right)| {
                let direction = instrs.next().unwrap();
                if direction == 'L' {
                    left
                } else {
                    right
                }
            })
            .unwrap();
        steps += 1;
        if current == "ZZZ" {
            break;
        }
    }

    steps
}

fn part2(input: &'static str) -> usize {
    let game = parse(input);
    let mut steps = 0;
    let mut currents: Vec<_> = game
        .net
        .keys()
        .cloned()
        .filter(|&k| k.ends_with('A'))
        .collect();
    let mut cycles: Vec<(usize, usize)> = vec![(0, 0); currents.len()];
    let mut instrs = game.instrs.chars().cycle();

    loop {
        let direction = instrs.next().unwrap();
        for (current, cycle) in currents.iter_mut().zip(cycles.iter_mut()) {
            *current = game
                .net
                .get(*current)
                .map(|&(left, right)| if direction == 'L' { left } else { right })
                .unwrap();

            if current.ends_with('Z') {
                *cycle = (steps + 1, cycle.0);
            }
        }
        steps += 1;

        if cycles.iter().all(|c| c.1 != 0) {
            break;
        }
    }

    cycles
        .into_iter()
        .map(|cycle| cycle.0 - cycle.1)
        .reduce(num::integer::lcm)
        .unwrap()
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
        let input = "LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)";
        assert_eq!(part1(input), 6);
    }

    #[test]
    fn test_part2() {
        let input = "LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)";
        assert_eq!(part2(input), 6);
    }
}
