use regex::Regex;
use std::collections::HashMap;
use std::io::{self, Read};

#[derive(Debug, Eq, Ord, PartialEq, PartialOrd)]
enum Action {
    Start(usize),
    Down,
    Up,
}

use self::Action::*;

#[derive(Debug, Eq, Ord, PartialEq, PartialOrd)]
struct Entry {
    // year: usize,
    month: usize,
    day: usize,
    hour: usize,
    minute: usize,
    action: Action,
}

const DAYS: [usize; 13] = [0, 31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];

impl Entry {
    fn timestamp(&self) -> usize {
        let days = DAYS.iter().skip(1).take(self.month).sum::<usize>() + self.day - 1;
        (days * 24 + self.hour) * 60 + self.minute
    }
}

fn main() -> io::Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let re = Regex::new(r"\[(?P<year>\d{4})-(?P<month>\d{2})-(?P<day>\d{2}) (?P<hour>\d{2}):(?P<minute>\d{2})\] (?P<message>.*)").unwrap();
    let message_re = Regex::new(r"Guard #(?P<num>\d+) begins shift").unwrap();

    let mut entries: Vec<Entry> = input
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| {
            let caps = re.captures(line).unwrap();
            Entry {
                // year: caps["year"].parse().unwrap(),
                month: caps["month"].parse().unwrap(),
                day: caps["day"].parse().unwrap(),
                hour: caps["hour"].parse().unwrap(),
                minute: caps["minute"].parse().unwrap(),
                action: match &caps["message"] {
                    "falls asleep" => Down,
                    "wakes up" => Up,
                    other => {
                        let matches = message_re.captures(other).unwrap();
                        Start(matches["num"].parse().unwrap())
                    }
                },
            }
        })
        .collect();

    entries.sort();
    println!("{entries:?}");
    println!("{:?}", entries.len());

    let mut guard = if let Start(g) = entries[0].action {
        g
    } else {
        panic!("Who's the first guard?");
    };

    let mut sleep_start = 0;
    let mut sleep: HashMap<usize, [usize; 60]> = HashMap::new();
    let mut sleep_total: HashMap<usize, usize> = HashMap::new();
    let mut max_sleeper = guard;
    let mut max_sleeper_sleep_total = 0;
    for entry in entries {
        match entry.action {
            Start(g) => guard = g,
            Down => {
                sleep_start = entry.timestamp();
            }
            Up => {
                let schedule = sleep.entry(guard).or_insert([0; 60]);
                let total = sleep_total.entry(guard).or_default();
                println!(
                    "guard: {} asleep for {}",
                    guard,
                    entry.timestamp() - sleep_start
                );
                for i in sleep_start..entry.timestamp() {
                    schedule[i % 60] += 1;
                    *total += 1;
                }
                if max_sleeper_sleep_total < *total {
                    max_sleeper = guard;
                    max_sleeper_sleep_total = *total;
                }
            }
        }
    }

    println!("{sleep_total:?}");

    for (k, v) in sleep.iter() {
        println!(
            "{}: {}",
            k,
            v.iter()
                .map(ToString::to_string)
                .collect::<Vec<_>>()
                .join(", ")
        );
    }

    println!("max_sleeper: {max_sleeper}");
    let mut max_sleep_minute = 0;
    let mut max_sleep_count = 0;
    let sleeper_sleep = sleep.get(&max_sleeper).unwrap();
    for (minute, sleeper_sleep_count) in sleeper_sleep.iter().enumerate().take(60) {
        if max_sleep_count < *sleeper_sleep_count {
            max_sleep_minute = minute;
            max_sleep_count = *sleeper_sleep_count;
        }
    }
    println!("max_minute: {max_sleep_minute}");
    println!("strategy 1 result: {}", max_sleeper * max_sleep_minute);

    for g in sleep.keys() {
        let sleeper_sleep = sleep.get(g).unwrap();
        for (minute, sleeper_sleep_count) in sleeper_sleep.iter().enumerate().take(60) {
            if max_sleep_count < *sleeper_sleep_count {
                max_sleeper = *g;
                max_sleep_minute = minute;
                max_sleep_count = *sleeper_sleep_count;
            }
        }
    }

    println!("max_sleeper: {max_sleeper}");
    println!("max_minute: {max_sleep_minute}");
    println!("strategy 2 result: {}", max_sleeper * max_sleep_minute);

    Ok(())
}
