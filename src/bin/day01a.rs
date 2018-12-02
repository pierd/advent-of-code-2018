use std::io::{self, Read};

fn main() -> io::Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;
    println!(
        "{}",
        input.split("\n").fold(0, |acc, line| if !line.is_empty() {
            let (sign, num) = line.split_at(1);
            acc + num.parse::<isize>().unwrap() * match sign {
                "-" => -1,
                "+" => 1,
                _ => panic!("WTF?"),
            }
        } else {
            acc
        })
    );
    Ok(())
}
