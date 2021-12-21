use std::collections::{HashMap, HashSet};
use std::io::{self, Read};

#[derive(Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct Range(usize, usize, usize);

#[derive(Debug, Eq, Ord, PartialEq, PartialOrd)]
struct Rect(Range, Range, usize);

#[derive(Debug)]
enum RangeSweep<'a> {
    On(&'a Range),
    Off(&'a Range),
}

use self::RangeSweep::{Off, On};

fn main() -> io::Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;
    let mut idx = 0; // a dummy solution to the problem of ranges being equal

    // parse everything
    let claims: Vec<Rect> = input
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| {
            // horrible parsing
            let mut coords_iter = line.split(" @ ").nth(1).unwrap().split(": ");
            let mut point_iter = coords_iter.next().unwrap().split(',');
            let mut size_iter = coords_iter.next().unwrap().split('x');
            let dim1 = point_iter.next().unwrap().parse().unwrap();
            let dim2 = point_iter.next().unwrap().parse().unwrap();
            let dim1_end = dim1 + size_iter.next().unwrap().parse::<usize>().unwrap();
            let dim2_end = dim2 + size_iter.next().unwrap().parse::<usize>().unwrap();
            idx += 1;
            let range1 = Range(dim1, dim1_end, idx);
            idx += 1;
            let range2 = Range(dim2, dim2_end, idx);
            idx += 1;
            Rect(range1, range2, idx)
        })
        .collect();

    // prepare edge points for first dimension
    let mut all_first_dimens = Vec::with_capacity(claims.len() * 2);
    let mut sweeps: HashMap<usize, Vec<RangeSweep>> = HashMap::new();
    for claim in &claims {
        all_first_dimens.push((claim.0).0);
        all_first_dimens.push((claim.0).1);
        sweeps.entry((claim.0).0).or_default().push(On(&claim.1));
        sweeps.entry((claim.0).1).or_default().push(Off(&claim.1));
    }
    all_first_dimens.sort_unstable();
    all_first_dimens.dedup();

    let mut overlapping = 0; // counter for overlapping area

    // currently active ranges (in terms of first dimension)
    let mut active_ranges: HashSet<&Range> = HashSet::new();
    for i in 0..all_first_dimens.len() {
        if i > 0 && !active_ranges.is_empty() {
            // something is active -> let's check the other dimension
            let hight = all_first_dimens[i] - all_first_dimens[i - 1];

            // prepare edge points for second dimension
            let mut strip_dimens = Vec::with_capacity(active_ranges.len() * 2);
            let mut strip_sweeps: HashMap<usize, Vec<RangeSweep>> = HashMap::new();
            let mut strip_active_ranges: HashSet<&Range> = HashSet::new();
            let mut strip_overlapping = 0;
            for range in active_ranges.iter() {
                strip_dimens.push(range.0);
                strip_dimens.push(range.1);
                strip_sweeps.entry(range.0).or_default().push(On(range));
                strip_sweeps.entry(range.1).or_default().push(Off(range));
            }
            strip_dimens.sort_unstable();
            strip_dimens.dedup();

            for j in 0..strip_dimens.len() {
                if j > 0 && strip_active_ranges.len() > 1 {
                    // more then 1 range is active -> we have an overlap!
                    let current_overlap = strip_dimens[j] - strip_dimens[j - 1];
                    strip_overlapping += hight * current_overlap;
                }
                // process changes for current edge point (in strip = second dimension)
                for range_sweep in strip_sweeps.get(&strip_dimens[j]).unwrap() {
                    match range_sweep {
                        On(range) => strip_active_ranges.insert(range),
                        Off(range) => strip_active_ranges.remove(range),
                    };
                }
            }
            overlapping += strip_overlapping;
        }

        // process changes for current edge point (first dimension)
        for range_sweep in sweeps.get(&all_first_dimens[i]).unwrap() {
            match range_sweep {
                On(range) => active_ranges.insert(range),
                Off(range) => active_ranges.remove(range),
            };
        }
    }

    println!("{}", overlapping);

    Ok(())
}
