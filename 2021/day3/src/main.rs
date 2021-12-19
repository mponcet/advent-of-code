const NR_BITS: usize = 12;

fn count_bits_at(lines: &[&str], idx: usize) -> (usize, usize) {
    let ones: usize = lines.iter().filter(|l| l.as_bytes()[idx] == b'1').count();

    (lines.len() - ones, ones)
}

fn part1() -> u32 {
    let input = include_str!("../input.txt");
    let lines: Vec<&str> = input.lines().collect();

    let mut gamma_rate = 0;
    for i in 0..NR_BITS {
        let (zero_count, one_count) = count_bits_at(&lines, i);
        if one_count > zero_count {
            gamma_rate |= 1 << (NR_BITS - 1 - i);
        }
    }

    gamma_rate * (!gamma_rate & ((1 << NR_BITS) - 1))
}

fn part2() -> u32 {
    let input = include_str!("../input.txt");

    let rating = |most_common: bool| -> u32 {
        let mut lines: Vec<_> = input.lines().collect();
        for i in 0..NR_BITS {
            let (zeroes, ones) = count_bits_at(&lines, i);
            let filter_char = match (most_common, zeroes > ones) {
                (true, true) | (false, false) => b'0',
                _ => b'1',
            };
            lines = lines
                .into_iter()
                .filter(|l| l.as_bytes()[i] == filter_char)
                .collect();

            if lines.len() == 1 {
                return u32::from_str_radix(lines.first().unwrap(), 2).unwrap();
            }
        }

        panic!("failure to get rating");
    };

    let oxgygen_gen_rate = rating(true);
    let co2_scrub_rate = rating(false);

    oxgygen_gen_rate * co2_scrub_rate
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
