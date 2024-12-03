use regex::Regex;

fn part1(input: &str) -> u32 {
    let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();

    re.captures_iter(input)
        .map(|cap| {
            let x = cap.get(1).unwrap().as_str().parse::<u32>().unwrap();
            let y = cap.get(2).unwrap().as_str().parse::<u32>().unwrap();
            x * y
        })
        .sum()
}

fn part2(input: &str) -> u32 {
    let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();

    re.captures_iter(input)
        .map(|cap| {
            let x = cap.get(1).unwrap().as_str().parse::<u32>().unwrap();
            let y = cap.get(2).unwrap().as_str().parse::<u32>().unwrap();
            let start = cap.get(0).unwrap().start();
            let last_do = input[0..start].rfind("do()");
            let last_dont = input[0..start].rfind("don't()");

            match (last_do, last_dont) {
                (Some(last_do), Some(last_dont)) if last_do > last_dont => x * y,
                (None, None) => x * y,
                (Some(_), None) => x * y,
                _ => 0,
            }
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

    #[test]
    fn test_part1() {
        let input = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
        assert_eq!(part1(input), 161);
    }

    #[test]
    fn test_part2() {
        let input = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";
        assert_eq!(part2(input), 48);
    }
}
