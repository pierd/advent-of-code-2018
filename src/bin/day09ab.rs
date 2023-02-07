use regex::Regex;
use std::fmt;
use std::io::{self, Read};

struct Cycle {
    next: Vec<Option<usize>>,
    prev: Vec<Option<usize>>,
}

impl Cycle {
    fn new(size: usize) -> Self {
        Cycle {
            next: vec![None; size],
            prev: vec![None; size],
        }
    }

    fn contains(&self, x: usize) -> bool {
        matches!(
            (self.next[x], self.prev[x]),
            (None, None) | (Some(_), Some(_))
        );
        self.next[x].is_some() && self.prev[x].is_some()
    }

    fn init(&mut self, x: usize) {
        assert!(!self.contains(x));
        self.next[x] = Some(x);
        self.prev[x] = Some(x);
    }

    fn forward(&self, x: usize, n: usize) -> usize {
        let mut x = x;
        for _ in 0..n {
            x = self.next[x].unwrap();
        }
        x
    }

    fn backwards(&self, x: usize, n: usize) -> usize {
        let mut x = x;
        for _ in 0..n {
            x = self.prev[x].unwrap();
        }
        x
    }

    fn insert_after(&mut self, x: usize, y: usize) {
        assert!(self.contains(x));
        assert!(!self.contains(y));

        // from: x <-> z
        // to: x <-> y <-> z
        let z = self.next[x].unwrap();

        // x <- y -> z
        self.prev[y] = Some(x);
        self.next[y] = Some(z);

        // x -> y <- z
        self.next[x] = Some(y);
        self.prev[z] = Some(y);
    }

    fn remove(&mut self, x: usize) -> usize {
        assert!(self.contains(x));

        // from: a <-> x <-> b
        // to: a <-> b
        let a = self.prev[x].unwrap();
        let b = self.next[x].unwrap();

        // a -> b
        self.next[a] = Some(b);

        // a <- b
        self.prev[b] = Some(a);

        // remove x
        self.next[x] = None;
        self.prev[x] = None;

        b
    }
}

impl fmt::Debug for Cycle {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.contains(0) {
            let mut s = String::new();
            s.push('0');
            let mut x = self.next[0].unwrap();
            while x != 0 {
                s.push(' ');
                s.push_str(&x.to_string());
                x = self.forward(x, 1);
            }
            write!(f, "{s}")
        } else {
            write!(f, "Empty")
        }
    }
}

fn solve(players: usize, last_marble: usize) -> usize {
    let mut cycle = Cycle::new(last_marble + 1);
    cycle.init(0);
    let mut points = vec![0usize; players];
    let mut current = 0;
    for turn in 1..=last_marble {
        if turn % 23 == 0 {
            let removed = cycle.backwards(current, 7);
            current = cycle.remove(removed);
            points[turn % players] += turn + removed;
        } else {
            cycle.insert_after(cycle.forward(current, 1), turn);
            current = turn;
        }
    }
    points.into_iter().max().unwrap()
}

fn main() -> io::Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let re = Regex::new(r"(\d+) players; last marble is worth (\d+) points").unwrap();
    let cases: Vec<(usize, usize)> = input
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| {
            let caps = re.captures(line).unwrap();
            (caps[1].parse().unwrap(), caps[2].parse().unwrap())
        })
        .collect();

    for (players, last_marble) in &cases {
        println!("{}", solve(*players, *last_marble));
    }

    Ok(())
}
