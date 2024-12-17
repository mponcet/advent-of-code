use regex::Regex;

#[derive(Debug)]
struct Equation {
    ax: i64,
    ay: i64,
    bx: i64,
    by: i64,
    px: i64,
    py: i64,
}

fn parse(input: &str) -> Vec<Equation> {
    fn numbers(line: &str) -> (i64, i64) {
        let re = Regex::new(r"(\d+)[^\d]+(\d+)").unwrap();
        let caps = re.captures(line).unwrap();
        (
            caps.get(1).unwrap().as_str().parse().unwrap(),
            caps.get(2).unwrap().as_str().parse().unwrap(),
        )
    }

    input
        .split("\n\n")
        .map(|game| {
            let mut lines = game.lines();
            let (ax, ay) = numbers(lines.next().unwrap());
            let (bx, by) = numbers(lines.next().unwrap());
            let (px, py) = numbers(lines.next().unwrap());
            Equation {
                ax,
                ay,
                bx,
                by,
                px,
                py,
            }
        })
        .collect()
}

// https://en.wikipedia.org/wiki/Cramer's_rule#Applications
fn solve(equation: &Equation) -> Option<(i64, i64)> {
    let (ax, ay, bx, by, px, py) = (
        equation.ax,
        equation.ay,
        equation.bx,
        equation.by,
        equation.px,
        equation.py,
    );

    // solve:
    // a*ax + b*bx = px
    // a*ay + b*by = py
    let det_div = ax * by - bx * ay;
    let det_a = px * by - bx * py;
    let det_b = ax * py - px * ay;
    if det_a % det_div == 0 && det_b % det_div == 0 {
        let a = det_a / det_div;
        let b = det_b / det_div;
        Some((a, b))
    } else {
        None
    }
}

fn part1(input: &str) -> usize {
    let equations = parse(input);

    equations
        .into_iter()
        .filter_map(|eq| solve(&eq))
        .map(|(a, b)| 3 * a + b)
        .sum::<i64>() as usize
}

fn part2(input: &str) -> usize {
    let equations = parse(input);

    equations
        .into_iter()
        .map(|eq| Equation {
            px: eq.px + 10000000000000,
            py: eq.py + 10000000000000,
            ..eq
        })
        .filter_map(|eq| solve(&eq))
        .map(|(a, b)| 3 * a + b)
        .sum::<i64>() as usize
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
        let input = "Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279";
        assert_eq!(part1(input), 480);
    }
}
