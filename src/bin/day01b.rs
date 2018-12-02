use std::collections::HashSet;
use std::io::{self, Read};

fn main() -> io::Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;
    let changes: Vec<isize> = input
        .split("\n")
        .filter(|line| !line.is_empty())
        .map(|line| {
            let (sign, num) = line.split_at(1);
            num.parse::<isize>().unwrap() * match sign {
                "-" => -1,
                "+" => 1,
                _ => panic!("WTF?"),
            }
        }).collect();
    let mut freq = 0;
    let mut seen = HashSet::new();
    'outer: loop {
        for n in &changes {
            freq += n;
            if (seen.contains(&freq)) {
                println!("{}", freq);
                break 'outer;
            }
            seen.insert(freq);
        }
    }
    Ok(())
}
