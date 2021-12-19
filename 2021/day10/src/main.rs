fn part1() -> usize {
    let lines = include_str!("../input.txt").lines();
    let mut error_score = 0;

    for line in lines {
        let mut stack = Vec::new();
        for c in line.chars() {
            match c {
                '(' => stack.push(')'),
                '{' => stack.push('}'),
                '[' => stack.push(']'),
                '<' => stack.push('>'),
                ')' | '}' | ']' | '>' => {
                    let elem = stack.pop().unwrap();
                    if elem != c {
                        error_score += match c {
                            ')' => 3,
                            ']' => 57,
                            '}' => 1197,
                            '>' => 25137,
                            _ => 0,
                        }
                    }
                }
                _ => panic!("Unexpected character"),
            }
        }
    }

    error_score
}

fn part2() -> usize {
    let lines = include_str!("../input.txt").lines();
    let mut scores = Vec::new();

    'line_loop: for line in lines {
        let mut stack: Vec<char> = Vec::new();
        for c in line.chars() {
            match c {
                '(' => stack.push(')'),
                '{' => stack.push('}'),
                '[' => stack.push(']'),
                '<' => stack.push('>'),
                ')' | '}' | ']' | '>' => {
                    let elem = stack.pop().unwrap();
                    if elem != c {
                        continue 'line_loop;
                    }
                }
                _ => (),
            }
        }

        let score = stack.into_iter().rev().fold(0, |acc, c| {
            acc * 5
                + match c {
                    ')' => 1,
                    ']' => 2,
                    '}' => 3,
                    '>' => 4,
                    _ => 0,
                }
        });
        scores.push(score);
    }

    let len = scores.len();
    *scores.select_nth_unstable(len / 2).1
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
