use regex::Regex;

fn part1(input: &str) -> u32 {
    let re = Regex::new(r"^\D*(\d).*?(\d)?\D*$").unwrap();

    input
        .lines()
        .map(|l| {
            let cap = re.captures(l).unwrap();
            let n1 = cap.get(1).unwrap().as_str().parse::<u32>().unwrap();
            let n2 = if let Some(n) = cap.get(2) {
                n.as_str().parse::<u32>().unwrap()
            } else {
                n1
            };
            n1 * 10 + n2
        })
        .sum()
}

fn part2(input: &str) -> u32 {
    let mut input = input.replace("one", "one1one");
    input = input.replace("two", "two2two");
    input = input.replace("three", "three3three");
    input = input.replace("four", "four4four");
    input = input.replace("five", "five5five");
    input = input.replace("six", "six6six");
    input = input.replace("seven", "seven7seven");
    input = input.replace("eight", "eight8eight");
    input = input.replace("nine", "nine9nine");
    part1(&input)
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
        let input = "1abc2\npqr3stu8vwx\na1b2c3d4e5f\ntreb7uchet";
        assert_eq!(part1(input), 142);
    }

    #[test]
    fn test_part2() {
        let input = "two1nine\neightwothree\nabcone2threexyz\nxtwone3four\n4nineeightseven2\nzoneight234\n7pqrstsixteen";
        assert_eq!(part2(input), 281);
    }
}
