use std::{cmp::Ordering, collections::HashMap};

fn parse(input: &str) -> Vec<(&[u8], Vec<usize>)> {
    input
        .lines()
        .map(|l| {
            l.split_once(' ')
                .map(|(record, groups)| {
                    (
                        record.as_bytes(),
                        groups.split(',').map(|n| n.parse().unwrap()).collect(),
                    )
                })
                .unwrap()
        })
        .collect()
}

fn dfs(
    cache: &mut HashMap<(usize, usize, usize), usize>,
    record: &[u8],
    groups: &[usize],
    mut from: usize,
    group: usize,
    mut pound_consumed: usize,
) -> usize {
    if from == record.len() {
        if group == groups.len() {
            return 1;
        } else {
            return 0;
        }
    }

    match record.get(from) {
        Some(b'.') => {
            if pound_consumed == 0 {
                let nr_dots = record[from..].iter().take_while(|&&c| c == b'.').count();
                dfs(cache, record, groups, from + nr_dots, group, 0)
            } else {
                0
            }
        }
        Some(b'#') => {
            if group == groups.len() {
                return 0;
            }

            let nr_pounds = record[from..].iter().take_while(|&&c| c == b'#').count();
            from += nr_pounds;
            pound_consumed += nr_pounds;

            match pound_consumed.cmp(&groups[group]) {
                Ordering::Equal => {
                    if from < record.len() {
                        dfs(cache, record, groups, from + 1, group + 1, 0)
                    } else {
                        dfs(cache, record, groups, from, group + 1, 0)
                    }
                }
                Ordering::Less => {
                    if from == record.len() {
                        0
                    } else {
                        dfs(cache, record, groups, from, group, pound_consumed)
                    }
                }
                Ordering::Greater => 0,
            }
        }
        Some(b'?') => {
            let mut ways = 0;

            if let Some(&cached_ways) = cache.get(&(group, from, pound_consumed)) {
                return cached_ways;
            }

            ways += if group < groups.len() {
                let mut record_pound = record.to_vec();
                record_pound[from] = b'#';
                dfs(cache, &record_pound, groups, from, group, pound_consumed)
            } else {
                0
            };

            let mut record_dot = record.to_vec();
            record_dot[from] = b'.';
            ways += dfs(cache, &record_dot, groups, from, group, pound_consumed);

            cache.insert((group, from, pound_consumed), ways);

            ways
        }
        _ => unreachable!(),
    }
}

fn part1(input: &str) -> usize {
    parse(input)
        .into_iter()
        .map(|(record, groups)| dfs(&mut HashMap::new(), record, &groups, 0, 0, 0))
        .sum()
}

fn part2(input: &str) -> usize {
    parse(input)
        .into_iter()
        .map(|(record, groups)| {
            let s = String::from_utf8(record.to_vec()).unwrap();
            let record_unfolded = format!("{s}?{s}?{s}?{s}?{s}",);
            let mut groups_unfolded = Vec::new();
            (0..5).for_each(|_| groups_unfolded.extend_from_slice(&groups));

            dfs(
                &mut HashMap::default(),
                record_unfolded.as_bytes(),
                &groups_unfolded,
                0,
                0,
                0,
            )
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
    const TEST_INPUT: &str = "???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1";

    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST_INPUT), 21);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(TEST_INPUT), 525152);
    }
}
