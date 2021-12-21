use regex::Regex;
use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap, HashSet};
use std::fmt;
use std::io::{self, Read};

struct ReverseOrdered<T>(T);

impl<T> fmt::Debug for ReverseOrdered<T>
where
    T: fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl<T> PartialOrd for ReverseOrdered<T>
where
    T: PartialOrd,
{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        other.0.partial_cmp(&self.0)
    }
}

impl<T> Ord for ReverseOrdered<T>
where
    T: Ord,
{
    fn cmp(&self, other: &Self) -> Ordering {
        other.0.cmp(&self.0)
    }
}

impl<T> PartialEq for ReverseOrdered<T>
where
    T: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        self.0.eq(&other.0)
    }
}

impl<T> Eq for ReverseOrdered<T> where T: Eq {}

fn solve_part1(entries: &[(String, String)]) -> String {
    let mut all_vertices = HashSet::new();
    let mut incoming: HashMap<String, HashSet<String>> = HashMap::new();
    let mut outgoing: HashMap<String, HashSet<String>> = HashMap::new();
    for (from, to) in entries {
        all_vertices.insert(from.to_string());
        all_vertices.insert(to.to_string());
        incoming
            .entry(to.to_string())
            .or_default()
            .insert(from.to_string());
        outgoing
            .entry(from.to_string())
            .or_default()
            .insert(to.to_string());
    }

    let mut answer = Vec::new();
    let mut ready: BinaryHeap<_> = all_vertices
        .iter()
        .filter(|v| incoming.entry(v.to_string()).or_default().is_empty())
        .map(|v| ReverseOrdered(v.to_string()))
        .collect();
    while !ready.is_empty() {
        let v = ready.pop().unwrap().0;
        answer.push(v.clone());
        if let Some(targets) = outgoing.get(&v) {
            for target in targets {
                let deps = incoming.entry(target.to_string()).or_default();
                deps.remove(&v);
                if deps.is_empty() {
                    ready.push(ReverseOrdered(target.to_string()));
                }
            }
        }
    }

    answer.join("")
}

fn work_time(label: &str) -> usize {
    (label.bytes().next().unwrap() - b'A' + 1) as usize
}

fn max(a: usize, b: usize) -> usize {
    if a < b {
        b
    } else {
        a
    }
}

fn solve_part2(entries: &[(String, String)]) -> usize {
    let (workers_count, base_time) = if entries.len() < 20 { (2, 0) } else { (5, 60) };

    let mut all_vertices = HashSet::new();
    let mut incoming: HashMap<String, HashSet<String>> = HashMap::new();
    let mut outgoing: HashMap<String, HashSet<String>> = HashMap::new();
    for (from, to) in entries {
        all_vertices.insert(from.to_string());
        all_vertices.insert(to.to_string());
        incoming
            .entry(to.to_string())
            .or_default()
            .insert(from.to_string());
        outgoing
            .entry(from.to_string())
            .or_default()
            .insert(to.to_string());
    }

    let mut earliest_start_time: HashMap<String, usize> = HashMap::new();

    let mut ready: Vec<_> = all_vertices
        .iter()
        .filter(|v| incoming.entry(v.to_string()).or_default().is_empty())
        .map(|v| ReverseOrdered((0, v.to_string())))
        .collect();

    let mut workers = BinaryHeap::new();
    for _ in 0..workers_count {
        workers.push(ReverseOrdered(0));
    }

    while !ready.is_empty() {
        ready.sort();
        let (t, v) = ready.pop().unwrap().0;
        let worker = workers.pop().unwrap().0;
        let start_time = max(t, worker);
        let finish_time = start_time + work_time(&v) + base_time;
        if let Some(targets) = outgoing.get(&v) {
            for target in targets {
                let deps = incoming.entry(target.to_string()).or_default();
                deps.remove(&v);
                let earliest = earliest_start_time.entry(target.to_string()).or_default();
                *earliest = max(*earliest, finish_time);
                if deps.is_empty() {
                    ready.push(ReverseOrdered((*earliest, target.to_string())));
                }
            }
        }
        workers.push(ReverseOrdered(finish_time));
    }

    workers.iter().min().unwrap().0
}

fn main() -> io::Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let re = Regex::new(r"Step ([A-Z]+) must be finished before step ([A-Z]+) can begin.").unwrap();

    let entries: Vec<(String, String)> = input
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| {
            let caps = re.captures(line).unwrap();
            ((&caps[1]).to_string(), (&caps[2]).to_string())
        })
        .collect();

    println!("{}", solve_part1(&entries));
    println!("{}", solve_part2(&entries));

    Ok(())
}
