use std::env;

struct Recipes {
    recipes: Vec<usize>,
    elf1: usize,
    elf2: usize,
    idx: usize,
}

impl Default for Recipes {
    fn default() -> Self {
        Recipes {
            recipes: vec![3, 7],
            elf1: 0,
            elf2: 1,
            idx: 0,
        }
    }
}

impl Iterator for Recipes {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        while self.recipes.len() <= self.idx {
            let new = self.recipes[self.elf1] + self.recipes[self.elf2];
            if new >= 10 {
                self.recipes.push(new / 10);
                self.recipes.push(new % 10);
            } else {
                self.recipes.push(new);
            }
            self.elf1 = (self.elf1 + self.recipes[self.elf1] + 1) % self.recipes.len();
            self.elf2 = (self.elf2 + self.recipes[self.elf2] + 1) % self.recipes.len();
        }
        self.idx += 1;
        Some(self.recipes[self.idx - 1])
    }
}

fn main() {
    let mut args = env::args();
    args.next(); // skip program name
    let arg = args.next().unwrap().parse::<usize>().unwrap();

    let recipes = Recipes::default();
    println!(
        "{}",
        recipes
            .skip(arg)
            .take(10)
            .map(|r| r.to_string())
            .collect::<String>()
    );

    let recipes = Recipes::default();
    let key: Vec<usize> = arg
        .to_string()
        .chars()
        .map(|c| c.to_string().parse().unwrap())
        .collect();
    let mut key_match = 0;
    for (i, r) in recipes.enumerate() {
        if r != key[key_match] {
            key_match = 0;
        }
        if r == key[key_match] {
            key_match += 1;
            if key_match == key.len() {
                println!("{}", i + 1 - key.len());
                break;
            }
        }
    }
}
