use std::io::{self, Read};

fn main() -> io::Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let mut overlapping = 0;
    let mut fabric: Vec<[usize; 1024]> = Vec::with_capacity(1024);
    for _ in 0..1024 {
        fabric.push([0; 1024]);
    }

    for (x, y, w, h) in input.lines().filter(|line| !line.is_empty()).map(|line| {
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
        for i in x..(x + w) {
            for j in y..(y + h) {
                fabric[i][j] += 1;
                if fabric[i][j] == 2 {
                    overlapping += 1;
                }
            }
        }
    }

    println!("{}", overlapping);

    Ok(())
}
