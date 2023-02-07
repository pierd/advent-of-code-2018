use aoc_helpers::{
    anyhow,
    scaffold::{solve, Problem, RowsOfChars},
};

struct Day15;

enum Tile {
    Wall,
    Open,
    Goblin,
    Elf,
}

impl TryFrom<char> for Tile {
    type Error = anyhow::Error;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '#' => Ok(Self::Wall),
            '.' => Ok(Self::Open),
            'G' => Ok(Self::Goblin),
            'E' => Ok(Self::Elf),
            _ => Err(anyhow::anyhow!("Can't parse: {:?}", value)),
        }
    }
}

struct Game {}

impl Game {
    fn from_map(_map: &[Vec<Tile>]) -> Self {
        Self {}
    }
}

impl Problem for Day15 {
    type Input = RowsOfChars<Tile>;
    type Part1 = usize;
    type Part2 = usize;

    fn solve_part1(input: &<Self::Input as aoc_helpers::scaffold::Parse>::Parsed) -> Self::Part1 {
        let _map = Game::from_map(input);
        // TODO
        Default::default()
    }

    fn solve_part2(_input: &<Self::Input as aoc_helpers::scaffold::Parse>::Parsed) -> Self::Part2 {
        Default::default()
    }
}

fn main() {
    solve::<Day15>(include_str!("../../inputs/day15.txt"));
}

#[cfg(test)]
mod tests {
    use super::*;
    use aoc_helpers::scaffold::{solve_part1, solve_part2};

    const SAMPLE: &str = "";

    #[test]
    fn test_sample() {
        assert_eq!(solve_part1::<Day15>(SAMPLE), Default::default());
        assert_eq!(solve_part2::<Day15>(SAMPLE), Default::default());
    }
}
