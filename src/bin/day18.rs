use std::collections::HashMap;

use aoc_helpers::{prelude::*, scaffold::RowsOfChars, tile_map};

struct Day18;

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
enum Tile {
    Open,
    Trees,
    Lumberyard,
}

impl TryFrom<char> for Tile {
    type Error = anyhow::Error;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '.' => Ok(Self::Open),
            '|' => Ok(Self::Trees),
            '#' => Ok(Self::Lumberyard),
            _ => Err(anyhow::anyhow!("Can't parse: {:?}", value)),
        }
    }
}

struct Stepper {
    steps: usize,
    seen_maps: HashMap<tile_map::TileMap<Tile>, usize>,
    current_step: usize,
}

impl Stepper {
    fn new_with_steps(steps: usize) -> Self {
        Self {
            steps,
            seen_maps: Default::default(),
            current_step: 0,
        }
    }
}

impl tile_map::Stepper<Tile> for Stepper {
    type Result = usize;

    fn apply_step_rule(&mut self, zone: [[Option<Tile>; 3]; 3]) -> Tile {
        let trees = tile_map::iter_all_neighbours(zone)
            .filter(|t| *t == Tile::Trees)
            .count();
        let lumberyards = tile_map::iter_all_neighbours(zone)
            .filter(|t| *t == Tile::Lumberyard)
            .count();
        let current = zone[1][1].expect("middle should always be present");
        match current {
            Tile::Open if trees >= 3 => Tile::Trees,
            Tile::Trees if lumberyards >= 3 => Tile::Lumberyard,
            Tile::Lumberyard if trees == 0 || lumberyards == 0 => Tile::Open,
            _ => current,
        }
    }

    fn step(&mut self, map: &tile_map::TileMap<Tile>) -> Option<Self::Result> {
        if let Some(prev_step) = self.seen_maps.get(map) {
            // cycle found
            let cycle_len = self.current_step - *prev_step;
            let cycles_left = (self.steps - self.current_step) / cycle_len;
            self.current_step += cycles_left * cycle_len;
        } else {
            // a new map -> store it
            self.seen_maps.insert(map.clone(), self.current_step);
        }

        // continue with regular stepping
        if self.current_step == self.steps {
            let trees = map.iter().filter(|t| *t == Tile::Trees).count();
            let lumberyards = map.iter().filter(|t| *t == Tile::Lumberyard).count();
            Some(trees * lumberyards)
        } else {
            self.current_step += 1;
            None
        }
    }
}

impl Problem for Day18 {
    type Input = RowsOfChars<Tile>;
    type Part1 = usize;
    type Part2 = usize;

    fn solve_part1(input: &<Self::Input as aoc_helpers::scaffold::Parse>::Parsed) -> Self::Part1 {
        tile_map::TileMap::<Tile>::from(input)
            .step(&mut Stepper::new_with_steps(10))
            .expect("stepping should finish")
    }

    fn solve_part2(input: &<Self::Input as aoc_helpers::scaffold::Parse>::Parsed) -> Self::Part2 {
        tile_map::TileMap::<Tile>::from(input)
            .step(&mut Stepper::new_with_steps(1000000000))
            .expect("stepping should finish")
    }
}

fn main() {
    solve::<Day18>(include_str!("../../inputs/day18.txt"));
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = ".#.#...|#.\n.....#|##|\n.|..|...#.\n..|#.....#\n#.#|||#|#|\n...#.||...\n.|....|...\n||...#|.#|\n|.||||..|.\n...#.|..|.";

    #[test]
    fn test_sample() {
        assert_eq!(solve_part1::<Day18>(SAMPLE), 1147);
    }
}
