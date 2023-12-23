use std::cell::{Cell, RefCell};
use std::collections::{HashMap, VecDeque};

#[derive(Debug)]
enum Module {
    Broadcaster {
        targets: Vec<String>,
    },
    FlipFlop {
        name: String,
        targets: Vec<String>,
        state: Cell<FlipFlopState>,
    },
    Conjonction {
        name: String,
        targets: Vec<String>,
        state: RefCell<ConjonctionInputsState>,
    },
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum FlipFlopState {
    On,
    Off,
}

type ConjonctionInputsState = HashMap<String, Pulse>;

#[derive(Clone, Copy, Debug, PartialEq)]
enum Pulse {
    Low,
    High,
}

fn parse(input: &str) -> HashMap<String, Module> {
    let modules: HashMap<_, _> = input
        .lines()
        .map(|l| {
            let (from, to) = l.split_once(" -> ").unwrap();
            let targets: Vec<_> = to.split(", ").map(|t| t.to_owned()).collect();
            match (&from[..1], &from[1..]) {
                ("b", "roadcaster") => ("broadcaster".to_owned(), Module::Broadcaster { targets }),
                ("%", name) => (
                    name.to_owned(),
                    Module::FlipFlop {
                        name: name.to_owned(),
                        targets,
                        state: Cell::new(FlipFlopState::Off),
                    },
                ),
                ("&", name) => (
                    name.to_owned(),
                    Module::Conjonction {
                        name: name.to_owned(),
                        targets,
                        state: RefCell::new(HashMap::new()),
                    },
                ),
                _ => unreachable!(),
            }
        })
        .collect();

    for (name, module) in &modules {
        let targets = match module {
            Module::Broadcaster { targets } => targets,
            Module::FlipFlop { targets, .. } => targets,
            Module::Conjonction { targets, .. } => targets,
        };

        for target in targets {
            if let Some(Module::Conjonction { state, .. }) = modules.get(target) {
                let mut state = state.borrow_mut();
                state.insert(name.to_owned(), Pulse::Low);
            }
        }
    }

    modules
}

fn press_button(modules: &HashMap<String, Module>) -> (usize, usize, Option<String>) {
    let mut low_pulses = 0;
    let mut high_pulses = 0;
    let mut ft_high_pulse_from = None;
    let mut queue = VecDeque::from([("", "broadcaster", Pulse::Low)]);

    while let Some((from, to, pulse)) = queue.pop_front() {
        match pulse {
            Pulse::Low => low_pulses += 1,
            Pulse::High => high_pulses += 1,
        }

        if to == "output" || to == "rx" {
            continue;
        }

        let module = modules.get(to).unwrap();
        match module {
            Module::Broadcaster { targets } => {
                targets
                    .iter()
                    .for_each(|target| queue.push_back(("", target, pulse)));
            }
            Module::FlipFlop {
                name,
                targets,
                state,
            } => match pulse {
                Pulse::Low => {
                    let new_pulse = if state.get() == FlipFlopState::On {
                        state.set(FlipFlopState::Off);
                        Pulse::Low
                    } else {
                        state.set(FlipFlopState::On);
                        Pulse::High
                    };
                    targets
                        .iter()
                        .for_each(|target| queue.push_back((name, target, new_pulse)));
                }
                Pulse::High => {}
            },
            Module::Conjonction {
                name,
                targets,
                state,
            } => {
                let mut state = state.borrow_mut();
                state.entry(from.to_string()).and_modify(|p| *p = pulse);
                let new_pulse = if state.values().all(|&p| p == Pulse::High) {
                    Pulse::Low
                } else {
                    Pulse::High
                };
                targets
                    .iter()
                    .for_each(|target| queue.push_back((name, target, new_pulse)));
                // part2
                if name == "ft" && pulse == Pulse::High {
                    ft_high_pulse_from = Some(from.to_owned());
                }
            }
        }
    }

    (low_pulses, high_pulses, ft_high_pulse_from)
}

fn part1(input: &str) -> usize {
    let modules = parse(input);

    let (low_pulses, high_pulses) = (0..1000)
        .map(|_| press_button(&modules))
        .fold((0, 0), |(low, high), (l, h, _)| (low + l, high + h));

    low_pulses * high_pulses
}

fn part2(input: &str) -> usize {
    let modules = parse(input);
    let mut cycles = HashMap::from([
        ("vz".to_owned(), 0),
        ("lt".to_owned(), 0),
        ("bq".to_owned(), 0),
        ("qh".to_owned(), 0),
    ]);

    for count in 1..5000usize {
        let (_, _, ft_high_pulse_from) = press_button(&modules);
        if let Some(src) = ft_high_pulse_from {
            cycles.insert(src, count);
        }
    }

    cycles.values().copied().reduce(num::integer::lcm).unwrap()
}

fn main() {
    println!("part1={}", part1(include_str!("../input.txt")));
    println!("part2={}", part2(include_str!("../input.txt")));
}

#[cfg(test)]
mod tests {
    use super::*;
    const TEST_INPUT: &str = "broadcaster -> a, b, c
%a -> b
%b -> c
%c -> inv
&inv -> a";
    const TEST_INPUT2: &str = "broadcaster -> a
%a -> inv, con
&inv -> b
%b -> con
&con -> output";

    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST_INPUT), 32000000);
        assert_eq!(part1(TEST_INPUT2), 11687500);
    }
}
