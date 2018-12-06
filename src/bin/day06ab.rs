use std::collections::VecDeque;
use std::io::{self, Read};
use std::ops::Range;

type Point = (isize, isize);
type PointIndex = usize;
type Distance = isize;

fn dist(a: Point, b: Point) -> Distance {
    (a.0 - b.0).abs() + (a.1 - b.1).abs()
}

struct Map {
    x_range: Range<isize>,
    y_range: Range<isize>,
    map: Vec<Vec<Vec<Option<Distance>>>>,
}

impl Map {
    fn new(x_range: Range<isize>, y_range: Range<isize>, points_count: PointIndex) -> Self {
        let mut map = Vec::with_capacity(x_range.size_hint().0);
        for _ in x_range.clone() {
            let mut row = Vec::with_capacity(y_range.size_hint().0);
            for _ in y_range.clone() {
                row.push(vec![None; points_count]);
            }
            map.push(row);
        }
        Map {
            x_range,
            y_range,
            map,
        }
    }

    fn is_in_bounds(&self, p: Point) -> bool {
        self.x_range.start <= p.0
            && p.0 < self.x_range.end
            && self.y_range.start <= p.1
            && p.1 < self.y_range.end
    }

    fn translate_point(&self, p: Point) -> Point {
        (p.0 - self.x_range.start, p.1 - self.y_range.start)
    }

    fn field_mut(&mut self, p: Point) -> &mut Vec<Option<Distance>> {
        let p = self.translate_point(p);
        &mut self.map[p.0 as usize][p.1 as usize]
    }

    fn visit(&mut self, p: Point, i: PointIndex, distance: Distance) -> bool {
        if !self.is_in_bounds(p) {
            return false;
        }
        let f = self.field_mut(p);
        if f[i] == None {
            f[i] = Some(distance);
            true
        } else if let Some(d) = f[i] {
            if distance < d {
                f[i] = Some(distance);
                true
            } else {
                false
            }
        } else {
            false
        }
    }
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

    // handle both example and real
    let limit = if points.len() < 10 { 32 } else { 10000 };

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

    let map_x_range = (min_x - max_dist)..(max_x + max_dist + 1);
    let map_y_range = (min_y - max_dist)..(max_y + max_dist + 1);
    let mut map = Map::new(map_x_range, map_y_range, points.len());

    let mut queue = VecDeque::new();
    for (i, p) in points.iter().enumerate() {
        map.field_mut(*p)[i] = Some(0);
        queue.push_back((i, *p, 0));
    }

    let mut in_queue_count = vec![1usize; points.len()];

    let mut last_distance = 0;
    while last_distance < limit && !queue.is_empty() {
        let (i, p, dist) = queue.pop_front().unwrap();
        in_queue_count[i] -= 1;
        last_distance = dist;
        let new_dist = dist + 1;
        for dp in [(0, 1), (1, 0), (0, -1), (-1, 0)].iter() {
            let new_p = (p.0 + dp.0, p.1 + dp.1);
            if map.visit(new_p, i, new_dist) {
                queue.push_back((i, new_p, new_dist));
                in_queue_count[i] += 1;
            }
        }
    }

    println!("{:?}", in_queue_count);

    // println!(
    //     "largest noninf area: {}",
    //     area.iter()
    //         .enumerate()
    //         .filter(|(i, _)| !infinite[*i])
    //         .map(|(_, a)| a)
    //         .max()
    //         .unwrap()
    // );

    // let mut zone = 0;
    // for x in (min_x - max_dist)..=(max_x + max_dist) {
    //     for y in (min_y - max_dist)..=(max_y + max_dist) {
    //         if sum_dists((x, y), &points) < limit {
    //             zone += 1;
    //         }
    //     }
    // }
    // println!("region size: {}", zone);

    Ok(())
}
