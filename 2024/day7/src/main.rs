struct Equation {
    result: usize,
    nums: Vec<usize>,
}

fn parse(input: &str) -> Vec<Equation> {
    input
        .lines()
        .map(|line| {
            let (result, nums) = line.split_once(": ").unwrap();

            Equation {
                result: result.parse().unwrap(),
                nums: nums.split(' ').map(|n| n.parse().unwrap()).collect(),
            }
        })
        .collect()
}

fn calc_r(left: usize, right: &[usize], op: char, result: usize, concat: bool) -> bool {
    if right.is_empty() {
        return false;
    }

    let left = match op {
        '+' => left + right[0],
        '*' => left * right[0],
        '|' => format!("{}{}", left, right[0]).parse().unwrap(),
        _ => unreachable!(),
    };

    if right.len() == 1 {
        return left == result;
    } else if left > result {
        return false;
    }

    calc_r(left, &right[1..], '+', result, concat)
        || calc_r(left, &right[1..], '*', result, concat)
        || (concat && calc_r(left, &right[1..], '|', result, concat))
}

fn part1(input: &str) -> usize {
    let equations = parse(input);

    equations
        .into_iter()
        .filter(|equation| {
            let Equation { result, ref nums } = *equation;

            calc_r(nums[0], &nums[1..], '+', result, false)
                || calc_r(nums[0], &nums[1..], '*', result, false)
        })
        .map(|equation| equation.result)
        .sum()
}

fn part2(input: &str) -> usize {
    let equations = parse(input);

    equations
        .into_iter()
        .filter(|equation| {
            let Equation { result, ref nums } = *equation;

            calc_r(nums[0], &nums[1..], '+', result, true)
                || calc_r(nums[0], &nums[1..], '*', result, true)
                || calc_r(nums[0], &nums[1..], '|', result, true)
        })
        .map(|equation| equation.result)
        .sum()
}

fn main() {
    println!("part1={}", part1(include_str!("../input.txt")));
    println!("part2={}", part2(include_str!("../input.txt")));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20";

    #[test]
    fn test_part1() {
        assert_eq!(part1(INPUT), 3749);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(INPUT), 11387);
    }
}
