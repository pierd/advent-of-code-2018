use regex::Regex;
use std::collections::{HashMap, HashSet};
use std::io::{self, Read};

type Registers = [usize; 4];
type Instruction = [usize; 4];

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
enum Arg {
    Register,
    Immediate,
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
enum Opcode {
    Add(Arg),
    Mul(Arg),
    Ban(Arg),
    Bor(Arg),
    Set(Arg),
    Gt(Arg, Arg),
    Eq(Arg, Arg),
}

impl Opcode {
    fn execute(&self, regs: &Registers, instr: &Instruction) -> Registers {
        use self::Opcode::*;
        let mut new_registers = regs.clone();
        new_registers[instr[3]] = match self {
            Add(arg) => regs[instr[1]] + Opcode::get_arg(regs, arg, instr[2]),
            Mul(arg) => regs[instr[1]] * Opcode::get_arg(regs, arg, instr[2]),
            Ban(arg) => regs[instr[1]] & Opcode::get_arg(regs, arg, instr[2]),
            Bor(arg) => regs[instr[1]] | Opcode::get_arg(regs, arg, instr[2]),
            Set(arg) => Opcode::get_arg(regs, arg, instr[1]),
            Gt(arg1, arg2) => {
                if Opcode::get_arg(regs, arg1, instr[1]) > Opcode::get_arg(regs, arg2, instr[2]) {
                    1
                } else {
                    0
                }
            }
            Eq(arg1, arg2) => {
                if Opcode::get_arg(regs, arg1, instr[1]) == Opcode::get_arg(regs, arg2, instr[2]) {
                    1
                } else {
                    0
                }
            }
        };
        new_registers
    }

    fn get_arg(registers: &Registers, arg: &Arg, num: usize) -> usize {
        match arg {
            Arg::Register => registers[num],
            Arg::Immediate => num,
        }
    }
}

struct Parser {
    before_re: Regex,
    instruction_re: Regex,
    after_re: Regex,
}

impl Default for Parser {
    fn default() -> Self {
        Parser {
            before_re: Regex::new(r"Before: *\[(\d+), *(\d+), *(\d+), *(\d+)\]").unwrap(),
            instruction_re: Regex::new(r"(\d+) +(\d+) +(\d+) +(\d+)").unwrap(),
            after_re: Regex::new(r"After: *\[(\d+), *(\d+), *(\d+), *(\d+)\]").unwrap(),
        }
    }
}

impl Parser {
    fn parse_before(&self, text: &str) -> Option<Registers> {
        Parser::parse(&self.before_re, text)
    }

    fn parse_instruction(&self, text: &str) -> Option<Instruction> {
        Parser::parse(&self.instruction_re, text)
    }

    fn parse_after(&self, text: &str) -> Option<Registers> {
        Parser::parse(&self.after_re, text)
    }

    fn parse(regex: &Regex, text: &str) -> Option<[usize; 4]> {
        match regex.captures(text) {
            None => None,
            Some(caps) => Some([
                caps[1].parse().unwrap(),
                caps[2].parse().unwrap(),
                caps[3].parse().unwrap(),
                caps[4].parse().unwrap(),
            ]),
        }
    }
}

#[derive(Debug)]
struct Entry {
    before: Registers,
    instruction: Instruction,
    after: Registers,
}

fn main() -> io::Result<()> {
    use self::Arg::*;
    use self::Opcode::*;

    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let parser = Parser::default();

    let mut lines = input.split("\n").filter(|line| !line.is_empty());

    let mut entries = Vec::new();
    let mut instructions = Vec::new();
    loop {
        if let Some(line) = lines.next() {
            match parser.parse_before(line) {
                None => {
                    instructions.push(parser.parse_instruction(line).unwrap());
                }
                Some(before) => {
                    let instruction = parser.parse_instruction(lines.next().unwrap()).unwrap();
                    let after = parser.parse_after(lines.next().unwrap()).unwrap();
                    entries.push(Entry {
                        before,
                        after,
                        instruction,
                    });
                }
            }
        } else {
            break;
        }
    }

    let all_opcodes = vec![
        Add(Register),
        Add(Immediate),
        Mul(Register),
        Mul(Immediate),
        Ban(Register),
        Ban(Immediate),
        Bor(Register),
        Bor(Immediate),
        Set(Register),
        Set(Immediate),
        Gt(Immediate, Register),
        Gt(Register, Immediate),
        Gt(Register, Register),
        Eq(Immediate, Register),
        Eq(Register, Immediate),
        Eq(Register, Register),
    ];

    let mut opcode_meaning: HashMap<usize, HashSet<Opcode>> = HashMap::with_capacity(16);
    let all_opcodes_set: HashSet<Opcode> = all_opcodes.iter().map(Clone::clone).collect();
    for i in 0..16 {
        opcode_meaning.insert(i, all_opcodes_set.clone());
    }
    let mut like_3_or_more = 0;
    for entry in entries.iter() {
        let mut matching = HashSet::new();
        for opcode in all_opcodes.iter() {
            let result = opcode.execute(&entry.before, &entry.instruction);
            if result == entry.after {
                matching.insert(*opcode);
            }
        }
        if matching.len() >= 3 {
            like_3_or_more += 1;
        }
        opcode_meaning
            .entry(entry.instruction[0])
            .or_default()
            .retain(|opcode| matching.contains(opcode));
    }
    println!("{}", like_3_or_more);

    // clean up the opcodes
    // grab the ones that the meaning is known and remove them from other ones
    let mut opcodes: HashMap<usize, Opcode> = opcode_meaning
        .iter()
        .filter(|(_, v)| v.len() == 1)
        .map(|(k, v)| (*k, *v.iter().next().unwrap()))
        .collect();
    while !instructions.is_empty() && opcodes.len() != opcode_meaning.len() {
        for (code, opcode) in opcodes.iter() {
            for (k, v) in opcode_meaning.iter_mut() {
                if k != code {
                    v.retain(|op| op != opcode);
                }
            }
        }
        opcodes.extend(
            opcode_meaning
                .iter()
                .filter(|(_, v)| v.len() == 1)
                .map(|(k, v)| (*k, *v.iter().next().unwrap())),
        );
    }

    println!(
        "{}",
        instructions.iter().fold([0, 0, 0, 0], |regs, instr| opcodes
            .get(&instr[0])
            .unwrap()
            .execute(&regs, instr))[0]
    );

    Ok(())
}
