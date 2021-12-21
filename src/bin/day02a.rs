use std::collections::HashMap;
use std::io::{self, Read};

fn main() -> io::Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;
    let (twos, threes) =
        input
            .lines()
            .filter(|line| !line.is_empty())
            .fold((0, 0), |(two, three), line| {
                let mut two_count = 0;
                let mut three_count = 0;
                let mut counts = HashMap::new();
                for chr in line.chars() {
                    match counts.entry(chr).and_modify(|x| *x += 1).or_insert(1) {
                        2 => {
                            two_count += 1;
                        }
                        3 => {
                            two_count -= 1;
                            three_count += 1;
                        }
                        4 => {
                            three_count -= 1;
                        }
                        _ => {}
                    }
                }
                (
                    if two_count > 0 { two + 1 } else { two },
                    if three_count > 0 { three + 1 } else { three },
                )
            });
    println!("{}", twos * threes);
    Ok(())
}
