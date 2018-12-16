use std::collections::HashMap;
use std::io::{self, Read};

type RulesMap = HashMap<(bool, bool, bool, bool, bool), bool>;

struct OffsettedVec {
    offset: isize,
    vec: Vec<bool>,
}

impl OffsettedVec {
    fn get(&self, idx: isize) -> bool {
        let idx = idx - self.offset;
        if idx < 0 || idx >= (self.vec.len() as isize) {
            false
        } else {
            self.vec[idx as usize]
        }
    }

    fn advance(self, rules: &RulesMap) -> Self {
        let mut first_offset = None;
        let mut vec = Vec::new();
        for idx in (self.offset - 2)..(self.offset + (self.vec.len() as isize) + 2) {
            let env = (
                self.get(idx - 2),
                self.get(idx - 1),
                self.get(idx),
                self.get(idx + 1),
                self.get(idx + 2),
            );
            let val = rules.get(&env).unwrap_or(&false);
            if *val || first_offset.is_some() {
                if first_offset.is_none() {
                    first_offset = Some(idx);
                }
                vec.push(*val);
            }
        }
        while vec.last() == Some(&false) {
            vec.pop();
        }
        OffsettedVec {
            offset: first_offset.unwrap_or_default(),
            vec: vec,
        }
    }

    fn score(&self) -> isize {
        let mut s = 0;
        for idx in self.offset..(self.offset + (self.vec.len() as isize)) {
            if self.get(idx) {
                s += idx;
            }
        }
        s
    }
}

#[allow(dead_code)]
fn print_out(v: &Vec<bool>) {
    for i in v {
        print!("{}", if *i { '#' } else { '.' });
    }
    println!("");
}

fn main() -> io::Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let mut lines = input.split("\n").filter(|line| !line.is_empty());

    let initial: Vec<bool> = lines
        .next()
        .unwrap()
        .chars()
        .skip("initial state: ".len())
        .map(|c| c == '#')
        .collect();

    let rules: RulesMap = lines
        .map(|line| {
            let mut parts = line.split(" => ");
            let mut first = parts.next().unwrap().chars().map(|c| c == '#');
            let last = parts.next().unwrap();
            (
                (
                    first.next().unwrap(),
                    first.next().unwrap(),
                    first.next().unwrap(),
                    first.next().unwrap(),
                    first.next().unwrap(),
                ),
                last.starts_with('#'),
            )
        })
        .collect();

    let mut world = OffsettedVec {
        vec: initial,
        offset: 0,
    };

    for _ in 0..20 {
        world = world.advance(&rules);
    }

    println!("{}", world.score());

    Ok(())
}
