use std::collections::HashSet;
use std::io::{self, Read};

fn react(a: char, b: char) -> bool {
    (a.is_ascii_uppercase() && b.is_ascii_lowercase() && a == b.to_ascii_uppercase())
        || (b.is_ascii_uppercase() && a.is_ascii_lowercase() && b == a.to_ascii_uppercase())
}

fn icase_eq(a: char, b: char) -> bool {
    a.to_ascii_lowercase() == b.to_ascii_lowercase()
}

fn reacted_length<T>(polymer: T) -> usize
where
    T: Iterator<Item = char>,
{
    let mut stack = Vec::new();
    for c in polymer {
        let reaction = match stack.last() {
            Some(x) => react(*x, c),
            None => false,
        };
        if reaction {
            stack.pop();
        } else {
            stack.push(c);
        }
    }
    stack.len()
}

fn main() -> io::Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;
    input = input.chars().filter(|x| (*x).is_alphabetic()).collect();

    println!("no removals: {}", reacted_length(input.chars()));

    let char_set: HashSet<char> = input.chars().map(|x| x.to_ascii_lowercase()).collect();

    println!(
        "with removals: {}",
        char_set
            .iter()
            .map(|c| reacted_length(input.chars().filter(|x| !icase_eq(*x, *c))))
            .min()
            .unwrap()
    );

    Ok(())
}
