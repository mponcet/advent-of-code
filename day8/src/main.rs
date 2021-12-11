fn parse_input() -> Vec<(Vec<u8>, Vec<u8>)> {
    include_str!("../input.txt")
        .lines()
        .map(|line| {
            let mut digits = line.split(" | ").map(|s| {
                s.split_ascii_whitespace()
                    // bitmap:
                    // number of segments can be calculated with bitmap.count_ones()
                    // bit 0 => segment 'a', segment bit 1 => 'b', etc
                    .map(|d| d.bytes().fold(0u8, |acc, x| acc | (1 << (x - b'a'))))
            });
            (
                digits.next().unwrap().collect(), // signal patterns
                digits.next().unwrap().collect(), // output values
            )
        })
        .collect()
}

fn part1() -> usize {
    parse_input()
        .into_iter()
        .flat_map(|(_, output)| output)
        .filter(|digit| match digit.count_ones() {
            2 | 3 | 4 | 7 => true, // numbers 1, 7, 4, 8
            _ => false,
        })
        .count()
}

fn find_by_cond<F>(patterns: &mut Vec<u8>, nr_segments: u32, cond: F) -> usize
where
    F: Fn(u8) -> bool,
{
    let index = patterns
        .iter()
        .position(|x| x.count_ones() == nr_segments && cond(*x))
        .unwrap();

    let digit = patterns[index];
    patterns.remove(index);
    digit as usize
}

fn part2() -> usize {
    let mut sum: usize = 0;

    for (mut patterns, output) in parse_input() {
        let mut digits_map = [0u8; 1 << 7];
        let one = find_by_cond(&mut patterns, 2, |_| true);
        digits_map[one] = 1;
        let four = find_by_cond(&mut patterns, 4, |_| true);
        digits_map[four] = 4;
        digits_map[find_by_cond(&mut patterns, 3, |_| true)] = 7;
        digits_map[find_by_cond(&mut patterns, 7, |_| true)] = 8;
        let six = find_by_cond(&mut patterns, 6, |x| ((one as u8) & x).count_ones() == 1);
        digits_map[six] = 6;
        digits_map[find_by_cond(&mut patterns, 6, |x| ((four as u8) & x).count_ones() == 4)] = 9;
        digits_map[find_by_cond(&mut patterns, 6, |_| true)] = 0;
        digits_map[find_by_cond(&mut patterns, 5, |x| ((six as u8) & x).count_ones() == 5)] = 5;
        digits_map[find_by_cond(&mut patterns, 5, |x| ((one as u8) & x).count_ones() == 2)] = 3;
        digits_map[find_by_cond(&mut patterns, 5, |_| true)] = 2;

        sum += output
            .iter()
            .fold(0, |acc, &x| acc * 10 + digits_map[x as usize] as usize);
    }

    sum
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
