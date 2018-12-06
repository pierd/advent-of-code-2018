use std::io::{self, Read};

fn main() -> io::Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;
    let ids: Vec<&str> = input.split("\n").filter(|line| !line.is_empty()).collect();
    for i in 0..ids.len() {
        for j in i..ids.len() {
            let diffs = ids[i]
                .chars()
                .into_iter()
                .zip(ids[j].chars().into_iter())
                .fold(
                    0,
                    |diffs, (c1, c2)| {
                        if c1 != c2 {
                            diffs + 1
                        } else {
                            diffs
                        }
                    },
                );
            if diffs == 1 {
                let mut result = String::new();
                for (c1, c2) in ids[i].chars().into_iter().zip(ids[j].chars().into_iter()) {
                    if c1 == c2 {
                        result.push(c1);
                    }
                }
                println!("{}", result);
            }
        }
    }
    Ok(())
}
