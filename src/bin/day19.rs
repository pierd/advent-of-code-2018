use std::str::FromStr;

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
enum Arg {
    Register,
    Immediate,
}

impl Arg {
    fn from_char(c: char) -> Option<Self> {
        match c {
            'r' => Some(Self::Register),
            'i' => Some(Self::Immediate),
            _ => None,
        }
    }

    fn get(&self, regs: &[usize; 6], num: usize) -> usize {
        match self {
            Arg::Register => regs[num],
            Arg::Immediate => num,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
enum Opcode {
    Add(Arg),
    Mul(Arg),
    Set(Arg),
    Gt(Arg, Arg),
    Eq(Arg, Arg),
}

impl FromStr for Opcode {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let arg = |c| Arg::from_char(c).ok_or(());
        let mut chars = s.chars();
        let mut next_char = || chars.next();
        match (next_char(), next_char(), next_char(), next_char()) {
            (Some('a'), Some('d'), Some('d'), Some(c)) => Ok(Opcode::Add(arg(c)?)),
            (Some('m'), Some('u'), Some('l'), Some(c)) => Ok(Opcode::Mul(arg(c)?)),
            (Some('s'), Some('e'), Some('t'), Some(c)) => Ok(Opcode::Set(arg(c)?)),
            (Some('g'), Some('t'), Some(c1), Some(c2)) => Ok(Opcode::Gt(arg(c1)?, arg(c2)?)),
            (Some('e'), Some('q'), Some(c1), Some(c2)) => Ok(Opcode::Eq(arg(c1)?, arg(c2)?)),
            _ => Err(()),
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct Instruction {
    opcode: Opcode,
    args: [usize; 3],
}

impl FromStr for Instruction {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split(' ');
        let opcode: Opcode = parts.next().unwrap().parse()?;
        let args = [
            parts.next().unwrap().parse().map_err(|_| ())?,
            parts.next().unwrap().parse().map_err(|_| ())?,
            parts.next().unwrap().parse().map_err(|_| ())?,
        ];
        Ok(Self { opcode, args })
    }
}

impl Instruction {
    fn execute(&self, regs: &mut [usize; 6]) {
        regs[self.args[2]] = match self.opcode {
            Opcode::Add(arg) => regs[self.args[0]] + arg.get(regs, self.args[1]),
            Opcode::Mul(arg) => regs[self.args[0]] * arg.get(regs, self.args[1]),
            Opcode::Set(arg) => arg.get(regs, self.args[0]),
            Opcode::Gt(arg1, arg2) => {
                if arg1.get(regs, self.args[0]) > arg2.get(regs, self.args[1]) {
                    1
                } else {
                    0
                }
            }
            Opcode::Eq(arg1, arg2) => {
                if arg1.get(regs, self.args[0]) == arg2.get(regs, self.args[1]) {
                    1
                } else {
                    0
                }
            }
        }
    }
}

fn parse(input: &str) -> (usize, Vec<Instruction>) {
    let mut lines = input.lines();
    let first_line = lines.next().unwrap();
    (
        first_line
            .strip_prefix("#ip ")
            .unwrap()
            .parse::<usize>()
            .unwrap(),
        lines
            .map(|line| line.parse::<Instruction>().unwrap())
            .collect::<Vec<Instruction>>(),
    )
}

fn execute_zeroed(ip_idx: usize, instructions: &[Instruction]) -> [usize; 6] {
    execute(Default::default(), ip_idx, instructions)
}

fn execute(mut regs: [usize; 6], ip_idx: usize, instructions: &[Instruction]) -> [usize; 6] {
    while let Some(instr) = instructions.get(regs[ip_idx]) {
        instr.execute(&mut regs);
        regs[ip_idx] += 1;
    }
    regs
}

fn hacked_calc(c: usize) -> usize {
    let mut a = 0;
    for b in 1..=c {
        if c % b == 0 {
            a += c / b;
        }
    }
    a
}

fn main() {
    let (ip_idx, mut instructions) = parse(include_str!("../../inputs/day19.txt"));
    let regs = execute_zeroed(ip_idx, &instructions);
    println!("Part 1: {}", regs[0]);

    // exit after initial processing and used handcrafted implementation instead
    instructions[1] = Instruction {
        opcode: Opcode::Set(Arg::Immediate),
        args: [100, 100, ip_idx],
    };
    let regs = execute([1, 0, 0, 0, 0, 0], ip_idx, &instructions);
    println!("Part 2: {}", hacked_calc(regs[2]));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_sample() {
        assert_eq!(
            parse(include_str!("../../inputs/day19-example.txt")),
            (
                0,
                vec![
                    Instruction {
                        opcode: Opcode::Set(Arg::Immediate),
                        args: [5, 0, 1]
                    },
                    Instruction {
                        opcode: Opcode::Set(Arg::Immediate),
                        args: [6, 0, 2]
                    },
                    Instruction {
                        opcode: Opcode::Add(Arg::Immediate),
                        args: [0, 1, 0]
                    },
                    Instruction {
                        opcode: Opcode::Add(Arg::Register),
                        args: [1, 2, 3]
                    },
                    Instruction {
                        opcode: Opcode::Set(Arg::Register),
                        args: [1, 0, 0]
                    },
                    Instruction {
                        opcode: Opcode::Set(Arg::Immediate),
                        args: [8, 0, 4]
                    },
                    Instruction {
                        opcode: Opcode::Set(Arg::Immediate),
                        args: [9, 0, 5]
                    },
                ]
            )
        )
    }

    #[test]
    fn test_execute_sample() {
        let (ip_idx, instructions) = parse(include_str!("../../inputs/day19-example.txt"));
        let regs = execute_zeroed(ip_idx, &instructions);
        assert_eq!(regs[0], 7);
    }
}
