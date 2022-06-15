use std::fmt::{Display, Write};

use aoc_helpers::{
    anyhow,
    scaffold::{solve, Problem, VecFromLines},
};
use rematch::rematch;

struct Day17;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Tile {
    Empty,
    StillWater,
    RunningWater,
    Sand,
}

impl Tile {
    fn supports_water(&self) -> bool {
        match self {
            Tile::Empty | Tile::RunningWater => false,
            Tile::StillWater | Tile::Sand => true,
        }
    }

    fn is_water(&self) -> bool {
        matches!(self, Tile::StillWater | Tile::RunningWater)
    }
}

struct Simulation {
    m: Vec<Vec<Tile>>,
    min_x: usize,
    min_y: usize,
}

impl From<&[Line]> for Simulation {
    fn from(lines: &[Line]) -> Self {
        let min_x = lines
            .iter()
            .map(|l| l.min_x())
            .min()
            .expect("there should be lines")
            - 1;
        let min_y = lines
            .iter()
            .map(|l| l.min_y())
            .min()
            .expect("there should be lines");
        let max_x = lines
            .iter()
            .map(|l| l.max_x())
            .max()
            .expect("there should be lines")
            + 1;
        let max_y = lines
            .iter()
            .map(|l| l.max_y())
            .max()
            .expect("there should be lines");
        let mut m = vec![vec![Tile::Empty; max_x - min_x + 1]; max_y - min_y + 1];
        for line in lines {
            match *line {
                Line::Horizontal { x, from_y, to_y } => {
                    for y in from_y..=to_y {
                        m[y - min_y][x - min_x] = Tile::Sand;
                    }
                }
                Line::Vertical { y, from_x, to_x } => {
                    for x in from_x..=to_x {
                        m[y - min_y][x - min_x] = Tile::Sand;
                    }
                }
            }
        }
        Self { m, min_x, min_y }
    }
}

impl Display for Simulation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in &self.m {
            for tile in row {
                f.write_char(match tile {
                    Tile::Empty => '.',
                    Tile::StillWater => '~',
                    Tile::RunningWater => '|',
                    Tile::Sand => '#',
                })?;
            }
            f.write_char('\n')?;
        }
        Ok(())
    }
}

impl Simulation {
    fn get(&self, row: isize, col: isize) -> Tile {
        if let (Ok(row), Ok(col)) = (usize::try_from(row), usize::try_from(col)) {
            if row < self.m.len() && col < self.m[row].len() {
                return self.m[row][col];
            }
        }
        Tile::Empty
    }

    fn get_surroundings(&self, row: isize, col: isize) -> [[Tile; 3]; 2] {
        [
            [
                self.get(row, col - 1),
                self.get(row, col),
                self.get(row, col + 1),
            ],
            [
                self.get(row + 1, col - 1),
                self.get(row + 1, col),
                self.get(row + 1, col + 1),
            ],
        ]
    }

    fn set(&mut self, row: isize, col: isize, tile: Tile) -> bool {
        if let (Ok(row), Ok(col)) = (usize::try_from(row), usize::try_from(col)) {
            if row < self.m.len() && col < self.m[row].len() {
                self.m[row][col] = tile;
                return true;
            }
        }
        false
    }

    fn pour(&mut self, x: usize, y: usize) {
        const THIS_ROW: usize = 0;
        const BELOW: usize = 1;
        const LEFT: usize = 0;
        const CENTER: usize = 1;
        const RIGHT: usize = 2;

        let mut stack = Vec::new();
        let row = [y, self.min_y].into_iter().min().unwrap() as isize;
        let col = x as isize - self.min_x as isize;
        self.set(row, col, Tile::RunningWater);
        stack.push((row, col));
        while let Some((row, col)) = stack.pop() {
            let zone = self.get_surroundings(row, col);

            // running and empty below -> make below running (and that's it)
            if zone[BELOW][CENTER] == Tile::Empty && self.set(row + 1, col, Tile::RunningWater) {
                stack.push((row + 1, col));
                continue;
            }

            // below supports water -> spill to sides
            if zone[BELOW][CENTER].supports_water() {
                if zone[THIS_ROW][LEFT] == Tile::Empty && self.set(row, col - 1, Tile::RunningWater)
                {
                    stack.push((row, col - 1));
                }
                if zone[THIS_ROW][RIGHT] == Tile::Empty
                    && self.set(row, col + 1, Tile::RunningWater)
                {
                    stack.push((row, col + 1));
                }

                // below supports water and either side is solid -> check for puddles
                if zone[THIS_ROW][LEFT] == Tile::Sand || zone[THIS_ROW][RIGHT] == Tile::Sand {
                    let mut left = col - 1;
                    while self.get(row, left) == Tile::RunningWater {
                        left -= 1;
                    }
                    let mut right = col + 1;
                    while self.get(row, right) == Tile::RunningWater {
                        right += 1;
                    }

                    // we have a puddle
                    if self.get(row, left) == Tile::Sand && self.get(row, right) == Tile::Sand {
                        for col in (left + 1)..right {
                            // all in between should be running water
                            assert_eq!(self.get(row, col), Tile::RunningWater);
                            // turn it into still water
                            self.set(row, col, Tile::StillWater);
                            // re-check all inflows
                            if self.get(row - 1, col) == Tile::RunningWater {
                                stack.push((row - 1, col));
                            }
                        }
                    }
                }
            }
        }
    }

    fn count_water(&self) -> usize {
        self.m
            .iter()
            .flat_map(|row| row.iter())
            .filter(|t| t.is_water())
            .count()
    }

    fn count_still_water(&self) -> usize {
        self.m
            .iter()
            .flat_map(|row| row.iter())
            .filter(|t| **t == Tile::StillWater)
            .count()
    }
}

#[derive(Clone, Copy, Debug)]
#[rematch]
enum Line {
    #[rematch(r"x=(\d+), y=(\d+)..(\d+)")]
    Horizontal {
        x: usize,
        from_y: usize,
        to_y: usize,
    },
    #[rematch(r"y=(\d+), x=(\d+)..(\d+)")]
    Vertical {
        y: usize,
        from_x: usize,
        to_x: usize,
    },
}

impl Line {
    fn min_x(&self) -> usize {
        match *self {
            Line::Horizontal { x, .. } => x,
            Line::Vertical { from_x, .. } => from_x,
        }
    }

    fn min_y(&self) -> usize {
        match *self {
            Line::Horizontal { from_y, .. } => from_y,
            Line::Vertical { y, .. } => y,
        }
    }

    fn max_x(&self) -> usize {
        match *self {
            Line::Horizontal { x, .. } => x,
            Line::Vertical { to_x, .. } => to_x,
        }
    }

    fn max_y(&self) -> usize {
        match *self {
            Line::Horizontal { to_y, .. } => to_y,
            Line::Vertical { y, .. } => y,
        }
    }
}

impl Problem for Day17 {
    type Input = VecFromLines<Line>;
    type Part1 = usize;
    type Part2 = usize;

    fn solve_part1(input: &<Self::Input as aoc_helpers::scaffold::Parse>::Parsed) -> Self::Part1 {
        let mut sim = Simulation::from(input.as_slice());
        // println!("{}", &sim);
        sim.pour(500, 0);
        let water = sim.count_water();
        println!("{}", &sim);
        water
    }

    fn solve_part2(input: &<Self::Input as aoc_helpers::scaffold::Parse>::Parsed) -> Self::Part2 {
        let mut sim = Simulation::from(input.as_slice());
        sim.pour(500, 0);
        sim.count_still_water()
    }
}

fn main() {
    solve::<Day17>(include_str!("../../inputs/day17.txt"));
}

#[cfg(test)]
mod tests {
    use super::*;
    use aoc_helpers::scaffold::{solve_part1, solve_part2};

    const SAMPLE: &str = "x=495, y=2..7\ny=7, x=495..501\nx=501, y=3..7\nx=498, y=2..4\nx=506, y=1..2\nx=498, y=10..13\nx=504, y=10..13\ny=13, x=498..504";

    #[test]
    fn test_sample() {
        assert_eq!(solve_part1::<Day17>(SAMPLE), 57);
        assert_eq!(solve_part2::<Day17>(SAMPLE), 29);
    }
}
