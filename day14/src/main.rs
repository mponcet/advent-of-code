type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

// Number of bits to store an element
const N: usize = 5;

fn pair_index(left: u8, right: u8) -> usize {
    let idx = ((left as usize) << N) | right as usize;
    idx
}

fn index_pair(index: usize) -> (u8, u8) {
    ((index >> N) as u8, (index & ((1 << N) - 1)) as u8)
}

fn parse_input() -> Result<(Vec<u8>, [u8; 1 << N * 2])> {
    let idx = |b| b - b'A';
    let (polymer, rules) = include_str!("../input.txt")
        .split_once("\n\n")
        .ok_or("missing rules")?;

    let polymer = polymer.bytes().map(idx).collect();
    let rules = rules
        .lines()
        .map(|l| l.as_bytes())
        .filter_map(|l| {
            let (left, right) = (idx(*l.get(0)?), idx(*l.get(1)?));
            let element = idx(*l.get(6)?);
            Some((pair_index(left, right), element))
        })
        .fold([0; 1 << N * 2], |mut r, (i, v)| {
            r[i] = v;
            r
        });

    Ok((polymer, rules))
}

fn polymer_run(polymer: &[u8], rules: &[u8; 1 << N * 2], steps: usize) -> Result<u64> {
    let mut pairs_counter = [0u64; 1 << N * 2];
    for pair in polymer.windows(2) {
        pairs_counter[pair_index(pair[0], pair[1])] += 1;
    }

    let mut element_counter = [0u64; 1 << N];
    for &element in polymer {
        element_counter[element as usize] += 1;
    }

    for _ in 0..steps {
        let mut new_pairs_counter = [0u64; 1 << N * 2];
        for (pair, &nr_pairs) in pairs_counter
            .iter()
            .enumerate()
            .filter(|(_, &nr_pairs)| nr_pairs > 0)
        {
            let element = rules[pair];
            let (left, right) = index_pair(pair);
            new_pairs_counter[pair_index(left, element)] += nr_pairs;
            new_pairs_counter[pair_index(element, right)] += nr_pairs;
            element_counter[element as usize] += nr_pairs;
        }
        pairs_counter = new_pairs_counter;
    }

    let (min, max) = element_counter
        .iter()
        .filter(|&&c| c != 0)
        .fold((u64::MAX, 0), |(min, max), &c| (min.min(c), max.max(c)));

    Ok(max - min)
}

fn part1() -> Result<u64> {
    let (polymer, rules) = parse_input()?;
    Ok(polymer_run(&polymer, &rules, 10)?)
}

fn part2() -> Result<u64> {
    let (polymer, rules) = parse_input()?;
    Ok(polymer_run(&polymer, &rules, 40)?)
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        println!("answer: {}", part1().unwrap());
    }

    #[test]
    fn test_part2() {
        println!("answer: {}", part2().unwrap());
    }
}
