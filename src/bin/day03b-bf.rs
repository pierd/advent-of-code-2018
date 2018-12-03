use std::collections::HashSet;
use std::io::{self, Read};

fn main() -> io::Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let mut not_overlapping = HashSet::new();
    let mut fabric: Vec<[usize; 1024]> = Vec::with_capacity(1024);
    for _ in 0..1024 {
        fabric.push([0; 1024]);
    }

    let mut idx = 1;
    for (x, y, w, h) in input
        .split("\n")
        .filter(|line| !line.is_empty())
        .map(|line| {
            // horrible parsing
            let mut coords_iter = line.split(" @ ").skip(1).next().unwrap().split(": ");
            let mut point_iter = coords_iter.next().unwrap().split(",");
            let mut size_iter = coords_iter.next().unwrap().split("x");
            let x = point_iter.next().unwrap().parse::<usize>().unwrap();
            let y = point_iter.next().unwrap().parse::<usize>().unwrap();
            let w = size_iter.next().unwrap().parse::<usize>().unwrap();
            let h = size_iter.next().unwrap().parse::<usize>().unwrap();
            (x, y, w, h)
        }) {
        not_overlapping.insert(idx);
        for i in x..(x + w) {
            for j in y..(y + h) {
                if fabric[i][j] != 0 {
                    not_overlapping.remove(&idx);
                    not_overlapping.remove(&fabric[i][j]);
                } else {
                    fabric[i][j] = idx;
                }
            }
        }
        idx += 1;
    }

    println!("{:?}", not_overlapping);

    Ok(())
}
