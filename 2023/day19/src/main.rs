use std::collections::HashMap;

#[derive(Debug)]
struct RuleEval {
    part: char,
    op: char,
    value: usize,
}

#[derive(Clone, Debug, PartialEq)]
enum RuleAction {
    Rule(String),
    Accept,
    Reject,
}

impl From<&str> for RuleAction {
    fn from(s: &str) -> Self {
        match s {
            "A" => RuleAction::Accept,
            "R" => RuleAction::Reject,
            _ => RuleAction::Rule(s.to_owned()),
        }
    }
}

#[derive(Debug)]
struct Rule {
    eval: Option<RuleEval>,
    action: RuleAction,
}

impl From<&str> for Rule {
    fn from(s: &str) -> Self {
        let pos_op = s.find('>').or_else(|| s.find('<'));
        let pos_action = s.find(':');
        match (pos_op, pos_action) {
            (None, None) => Rule {
                eval: None,
                action: RuleAction::from(s),
            },
            (Some(pos_op), Some(pos_action)) => {
                let part = s.chars().next().unwrap();
                let op = s.chars().nth(pos_op).unwrap();
                let value = s[pos_op + 1..pos_action].parse::<usize>().unwrap();
                let action = &s[pos_action + 1..];

                Rule {
                    eval: Some(RuleEval { part, op, value }),
                    action: RuleAction::from(action),
                }
            }
            _ => unreachable!(),
        }
    }
}

#[derive(Debug)]
struct Part {
    name: char,
    value: usize,
}

fn parse(input: &str) -> (HashMap<String, Vec<Rule>>, Vec<[Part; 4]>) {
    let (rules, parts) = input.split_once("\n\n").unwrap();

    let rules: HashMap<_, _> = rules
        .lines()
        .map(|l| {
            let (name, rules) = l.split_once('{').unwrap();

            let rules: Vec<_> = rules[..rules.len() - 1]
                .split(',')
                .map(Rule::from)
                .collect();

            (name.to_owned(), rules)
        })
        .collect();

    let parts: Vec<[Part; 4]> = parts
        .lines()
        .map(|l| {
            let s = &l[1..l.len() - 1];
            s.splitn(4, ',')
                .map(|p| Part {
                    name: p.chars().next().unwrap(),
                    value: p[2..].parse().unwrap(),
                })
                .collect::<Vec<_>>()
                .try_into()
                .unwrap()
        })
        .collect();

    (rules, parts)
}

fn eval_xmas_parts(parts: &[Part; 4], rules: &[Rule]) -> RuleAction {
    for rule in rules {
        if let Some(eval) = &rule.eval {
            let part = parts.iter().find(|p| p.name == eval.part).unwrap();

            match eval.op {
                '>' => {
                    if part.value > eval.value {
                        return rule.action.clone();
                    }
                }
                '<' => {
                    if part.value < eval.value {
                        return rule.action.clone();
                    }
                }
                _ => unreachable!(),
            }
        } else {
            return rule.action.clone();
        }
    }

    unreachable!()
}

fn part1(input: &str) -> usize {
    let mut result = 0;
    let (workflows, parts) = parse(input);

    for xmas in parts {
        let mut rules = workflows.get("in").unwrap();
        loop {
            let action = eval_xmas_parts(&xmas, rules);

            if let RuleAction::Rule(next) = action {
                rules = workflows.get(&next).unwrap();
            } else {
                if action == RuleAction::Accept {
                    result += xmas.iter().map(|p| p.value).sum::<usize>();
                }
                break;
            }
        }
    }

    result
}

const MIN_RATING: usize = 1;
const MAX_RATING: usize = 4000;

fn combinations(
    workflows: &HashMap<String, Vec<Rule>>,
    workflow: &str,
    xmas: [(usize, usize); 4],
) -> usize {
    let rules = workflows.get(workflow).unwrap();
    let mut exclude_xmas = xmas;

    rules
        .iter()
        .map(|rule| {
            let mut include_xmas = exclude_xmas;
            if let Some(eval) = &rule.eval {
                let (range_incl, range_excl) = match eval.part {
                    'x' => (&mut include_xmas[0], &mut exclude_xmas[0]),
                    'm' => (&mut include_xmas[1], &mut exclude_xmas[1]),
                    'a' => (&mut include_xmas[2], &mut exclude_xmas[2]),
                    's' => (&mut include_xmas[3], &mut exclude_xmas[3]),
                    _ => unreachable!(),
                };

                // x>1000 gives two ranges : (1001, 4000) and (0, 1000)
                // first one gets passed to the call chain (recursion)
                // second one is for the next iteration
                if eval.op == '<' {
                    *range_excl = (eval.value, range_excl.1.max(eval.value));
                    *range_incl = (range_incl.0, range_incl.1.min(eval.value - 1));
                } else {
                    *range_excl = (range_incl.0, eval.value.min(range_excl.1));
                    *range_incl = (range_incl.0.max(eval.value + 1), range_incl.1);
                }
            }

            match &rule.action {
                RuleAction::Accept => include_xmas
                    .iter()
                    .map(|(min, max)| max - min + 1)
                    .product(),
                RuleAction::Reject => 0,
                RuleAction::Rule(next) => combinations(workflows, next, include_xmas),
            }
        })
        .sum()
}

fn part2(input: &str) -> usize {
    let (workflows, _) = parse(input);

    combinations(&workflows, "in", [(MIN_RATING, MAX_RATING); 4])
}

fn main() {
    println!("part1={}", part1(include_str!("../input.txt")));
    println!("part2={}", part2(include_str!("../input.txt")));
}

#[cfg(test)]
mod tests {
    use super::*;
    const TEST_INPUT: &str = "px{a<2006:qkq,m>2090:A,rfg}
pv{a>1716:R,A}
lnx{m>1548:A,A}
rfg{s<537:gd,x>2440:R,A}
qs{s>3448:A,lnx}
qkq{x<1416:A,crn}
crn{x>2662:A,R}
in{s<1351:px,qqz}
qqz{s>2770:qs,m<1801:hdj,R}
gd{a>3333:R,R}
hdj{m>838:A,pv}

{x=787,m=2655,a=1222,s=2876}
{x=1679,m=44,a=2067,s=496}
{x=2036,m=264,a=79,s=2244}
{x=2461,m=1339,a=466,s=291}
{x=2127,m=1623,a=2188,s=1013}";

    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST_INPUT), 19114);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(TEST_INPUT), 167409079868000);
    }
}
