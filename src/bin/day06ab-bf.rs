use std::io::{self, Read};
use std::iter::repeat;

type Point = (isize, isize);

fn dist(a: Point, b: Point) -> isize {
    (a.0 - b.0).abs() + (a.1 - b.1).abs()
}

fn find_owner(x: Point, points: &Vec<Point>) -> Option<usize> {
    let mut result = None;
    let mut min_dist = 10000;
    for (i, p) in points.iter().enumerate() {
        let d = dist(x, *p);
        if d < min_dist {
            min_dist = d;
            result = Some(i);
        } else if d == min_dist {
            result = None;
        }
    }
    result
}

fn sum_dists(x: Point, points: &Vec<Point>) -> isize {
    points.iter().map(|p| dist(x, *p)).sum()
}

fn main() -> io::Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;
    let points: Vec<Point> = input
        .split("\n")
        .filter(|x| !x.is_empty())
        .map(|line| {
            let mut coords_iter = line.split(", ");
            (
                coords_iter.next().unwrap().parse::<isize>().unwrap(),
                coords_iter.next().unwrap().parse::<isize>().unwrap(),
            )
        }).collect();

    let max_dist = points
        .iter()
        .enumerate()
        .map(|(i, p1)| {
            points
                .split_at(i + 1)
                .1
                .iter()
                .map(|p2| dist(*p1, *p2))
                .max()
        }).max()
        .unwrap()
        .unwrap();

    let min_x = points.iter().map(|(a, _)| a).min().unwrap();
    let min_y = points.iter().map(|(_, a)| a).min().unwrap();
    let max_x = points.iter().map(|(a, _)| a).max().unwrap();
    let max_y = points.iter().map(|(_, a)| a).max().unwrap();

    let mut infinite: Vec<bool> = repeat(false).take(points.len()).collect();

    for x in (min_x - max_dist)..=(max_x + max_dist) {
        if let Some(i) = find_owner((x, min_y - max_dist), &points) {
            infinite[i] = true;
        }
        if let Some(i) = find_owner((x, max_y + max_dist), &points) {
            infinite[i] = true;
        }
    }

    for y in (min_y - max_dist)..=(max_y + max_dist) {
        if let Some(i) = find_owner((min_x - max_dist, y), &points) {
            infinite[i] = true;
        }
        if let Some(i) = find_owner((max_x + max_dist, y), &points) {
            infinite[i] = true;
        }
    }

    let mut area: Vec<usize> = repeat(0).take(points.len()).collect();
    for x in (min_x - max_dist)..=(max_x + max_dist) {
        for y in (min_y - max_dist)..=(max_y + max_dist) {
            if let Some(i) = find_owner((x, y), &points) {
                area[i] += 1;
            }
        }
    }

    println!(
        "largest noninf area: {}",
        area.iter()
            .enumerate()
            .filter(|(i, _)| !infinite[*i])
            .map(|(_, a)| a)
            .max()
            .unwrap()
    );

    // handle both example and real
    let limit = if points.len() < 10 { 32 } else { 10000 };
    let mut zone = 0;
    for x in (min_x - max_dist)..=(max_x + max_dist) {
        for y in (min_y - max_dist)..=(max_y + max_dist) {
            if sum_dists((x, y), &points) < limit {
                zone += 1;
            }
        }
    }
    println!("region size: {}", zone);

    Ok(())
}
