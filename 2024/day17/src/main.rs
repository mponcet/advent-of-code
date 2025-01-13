#[derive(Debug, Clone, Copy)]
enum Opcode {
    Adv,
    Bxl,
    Bst,
    Jnz,
    Bxc,
    Out,
    Bdv,
    Cdv,
}

impl TryFrom<usize> for Opcode {
    type Error = &'static str;

    fn try_from(value: usize) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Opcode::Adv),
            1 => Ok(Opcode::Bxl),
            2 => Ok(Opcode::Bst),
            3 => Ok(Opcode::Jnz),
            4 => Ok(Opcode::Bxc),
            5 => Ok(Opcode::Out),
            6 => Ok(Opcode::Bdv),
            7 => Ok(Opcode::Cdv),
            _ => Err("unexpected opcode"),
        }
    }
}

impl From<Opcode> for usize {
    fn from(value: Opcode) -> Self {
        match value {
            Opcode::Adv => 0,
            Opcode::Bxl => 1,
            Opcode::Bst => 2,
            Opcode::Jnz => 3,
            Opcode::Bxc => 4,
            Opcode::Out => 5,
            Opcode::Bdv => 6,
            Opcode::Cdv => 7,
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct Regs {
    a: usize,
    b: usize,
    c: usize,
    ip: usize,
}

#[derive(Debug)]
struct Instr {
    opcode: Opcode,
    operand: usize,
}

fn run(instrs: &[Instr], regs: &mut Regs, stop_at_out: bool) -> Vec<usize> {
    let mut output = Vec::new();

    while regs.ip < instrs.len() {
        let instr = &instrs[regs.ip];
        let combo = match instr.operand {
            value @ 0..=3 => value,
            4 => regs.a,
            5 => regs.b,
            6 => regs.c,
            _ => unreachable!(),
        };

        match instr.opcode {
            Opcode::Adv => regs.a /= 2usize.pow(combo as u32),
            Opcode::Bxl => regs.b ^= instr.operand,
            Opcode::Bst => regs.b = combo % 8,
            Opcode::Jnz => {
                if regs.a != 0 {
                    regs.ip = 0;
                    continue;
                }
            }
            Opcode::Bxc => regs.b ^= regs.c,
            Opcode::Out => {
                output.push(combo % 8);
                if stop_at_out {
                    break;
                }
            }
            Opcode::Bdv => regs.b = regs.a / 2usize.pow(combo as u32),
            Opcode::Cdv => regs.c = regs.a / 2usize.pow(combo as u32),
        }

        regs.ip += 1;
    }

    output
}

fn parse(input: &str) -> (Regs, Vec<Instr>) {
    let (regs, instrs) = input.split_once("\n\n").unwrap();

    let mut regs = regs.lines();
    let a = regs.next().unwrap()[12..].parse().unwrap();
    let b = regs.next().unwrap()[12..].parse().unwrap();
    let c = regs.next().unwrap()[12..].parse().unwrap();
    let regs = Regs { a, b, c, ip: 0 };

    let instrs = instrs[9..]
        .trim_end()
        .split(',')
        .collect::<Vec<_>>()
        .chunks(2)
        .map(|c| Instr {
            opcode: Opcode::try_from(c[0].parse::<usize>().unwrap()).unwrap(),
            operand: c[1].parse().unwrap(),
        })
        .collect();

    (regs, instrs)
}

fn part1(input: &str) -> String {
    let (mut regs, instrs) = parse(input);
    let output = run(&instrs, &mut regs, false);

    output
        .iter()
        .map(|n| n.to_string())
        .collect::<Vec<_>>()
        .join(",")
}

fn dfs(instrs: &[Instr], a: usize, program: &[usize]) -> usize {
    if program.is_empty() {
        return a;
    }

    for delta in 0..8 {
        let new_a = (a << 3) | delta;
        let mut regs = Regs {
            a: new_a,
            b: 0,
            c: 0,
            ip: 0,
        };
        let output = run(instrs, &mut regs, true);
        if output.first() == program.last() {
            let result = dfs(instrs, new_a, &program[..program.len() - 1]);

            if result != 0 {
                return result;
            }
        }
    }

    0
}

fn part2(input: &str) -> usize {
    let (_, instrs) = parse(input);
    let program = instrs
        .iter()
        .flat_map(|instr| [instr.opcode.into(), instr.operand])
        .collect::<Vec<_>>();

    dfs(&instrs, 0, &program)
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
        let input = "Register A: 729
Register B: 0
Register C: 0

Program: 0,1,5,4,3,0";
        assert_eq!(part1(input), "4,6,3,5,6,3,5,2,1,0");
    }

    #[test]
    fn test_part2() {
        let input = "Register A: 2024
Register B: 0
Register C: 0

Program: 0,3,5,4,3,0";
        assert_eq!(part2(input), 117440);
    }
}
