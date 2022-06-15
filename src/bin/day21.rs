use std::{cell::RefCell, collections::HashSet, str::FromStr};

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

#[derive(Clone, Debug)]
enum Opcode {
    Add(Arg),
    Mul(Arg),
    Ban(Arg),
    Bor(Arg),
    Set(Arg),
    Gt(Arg, Arg),
    Eq(Arg, Arg),
    Trap(Trap),
}

#[derive(Clone, Debug)]
struct Trap {
    seen: RefCell<HashSet<usize>>,
    last_before_seen: RefCell<usize>,
}

impl Trap {
    fn new() -> Self {
        Self {
            seen: Default::default(),
            last_before_seen: Default::default(),
        }
    }

    fn trap(&self, regs: &[usize; 6], args: [usize; 3]) -> usize {
        let interesting = regs[1];
        if self.seen.borrow_mut().insert(interesting) {
            *self.last_before_seen.borrow_mut() = interesting;
        } else {
            return 1;
        }
        if regs[args[0]] == regs[args[1]] {
            1
        } else {
            0
        }
    }
}

impl FromStr for Opcode {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let arg = |c| Arg::from_char(c).ok_or(());
        let mut chars = s.chars();
        let mut next_char = || chars.next().ok_or(());
        match (next_char()?, next_char()?, next_char()?, next_char()?) {
            ('a', 'd', 'd', c) => Ok(Opcode::Add(arg(c)?)),
            ('m', 'u', 'l', c) => Ok(Opcode::Mul(arg(c)?)),
            ('b', 'a', 'n', c) => Ok(Opcode::Ban(arg(c)?)),
            ('b', 'o', 'r', c) => Ok(Opcode::Bor(arg(c)?)),
            ('s', 'e', 't', c) => Ok(Opcode::Set(arg(c)?)),
            ('g', 't', c1, c2) => Ok(Opcode::Gt(arg(c1)?, arg(c2)?)),
            ('e', 'q', c1, c2) => Ok(Opcode::Eq(arg(c1)?, arg(c2)?)),
            _ => Err(()),
        }
    }
}

#[derive(Clone, Debug)]
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
        regs[self.args[2]] = match &self.opcode {
            Opcode::Add(arg) => regs[self.args[0]] + arg.get(regs, self.args[1]),
            Opcode::Mul(arg) => regs[self.args[0]] * arg.get(regs, self.args[1]),
            Opcode::Ban(arg) => regs[self.args[0]] & arg.get(regs, self.args[1]),
            Opcode::Bor(arg) => regs[self.args[0]] | arg.get(regs, self.args[1]),
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
            Opcode::Trap(trap) => trap.trap(regs, self.args),
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

fn execute(mut regs: [usize; 6], ip_idx: usize, instructions: &[Instruction]) -> [usize; 6] {
    while let Some(instr) = instructions.get(regs[ip_idx]) {
        instr.execute(&mut regs);
        regs[ip_idx] += 1;
    }
    regs
}

fn main() {
    let (ip_idx, instructions) = parse(include_str!("../../inputs/day21.txt"));

    {
        let mut hacked_instructions = instructions.clone();
        if let Opcode::Eq(Arg::Register, Arg::Register) = hacked_instructions[28].opcode {
            hacked_instructions[28] = Instruction {
                opcode: Opcode::Add(Arg::Immediate),
                args: [2, 100, 2],
            };
            let regs = execute([0, 0, 0, 0, 0, 0], ip_idx, &hacked_instructions);
            println!("Part 1: {}", regs[1]);
        } else {
            panic!("Can't hack it");
        }
    }

    {
        let mut hacked_instructions = instructions;
        if let Opcode::Eq(Arg::Register, Arg::Register) = hacked_instructions[28].opcode {
            hacked_instructions[28].opcode = Opcode::Trap(Trap::new());
            execute([0, 0, 0, 0, 0, 0], ip_idx, &hacked_instructions);
            if let Opcode::Trap(trap) = &hacked_instructions[28].opcode {
                println!("Part 2: {}", trap.last_before_seen.borrow());
            }
        } else {
            panic!("Can't hack it");
        }
    }
}
