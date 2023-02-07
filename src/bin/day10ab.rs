use regex::Regex;
use std::collections::HashSet;
use std::io::{self, Read};

fn print_out(points: &[(isize, isize, isize, isize)]) -> bool {
    let min_x = points.iter().map(|(n, _, _, _)| *n).min().unwrap();
    let max_x = points.iter().map(|(n, _, _, _)| *n).max().unwrap();
    let min_y = points.iter().map(|(_, n, _, _)| *n).min().unwrap();
    let max_y = points.iter().map(|(_, n, _, _)| *n).max().unwrap();
    if (max_x - min_x).abs() < 100 && (max_y - min_y).abs() < 15 {
        let points_set: HashSet<(isize, isize)> =
            points.iter().map(|(x, y, _, _)| (*x, *y)).collect();
        for y in min_y..=max_y {
            for x in min_x..=max_x {
                print!(
                    "{}",
                    if points_set.contains(&(x, y)) {
                        '#'
                    } else {
                        '.'
                    }
                );
            }
            println!();
        }
        true
    } else {
        false
    }
}

fn advance(points: &mut [(isize, isize, isize, isize)]) {
    for (x, y, dx, dy) in points.iter_mut() {
        *x += *dx;
        *y += *dy;
    }
}

fn main() -> io::Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let re = Regex::new(
        r"position=< *(?P<x>-?\d+), *(?P<y>-?\d+)> velocity=< *(?P<dx>-?\d+), *(?P<dy>-?\d+)>",
    )
    .unwrap();
    let mut points: Vec<(isize, isize, isize, isize)> = input
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| {
            let caps = re.captures(line).unwrap();
            (
                caps["x"].parse().unwrap(),
                caps["y"].parse().unwrap(),
                caps["dx"].parse().unwrap(),
                caps["dy"].parse().unwrap(),
            )
        })
        .collect();

    for t in 0..15000 {
        if print_out(&points) {
            println!("{t}");
            println!();
        }
        advance(&mut points);
    }

    Ok(())
}
