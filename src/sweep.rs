use std::collections::{HashMap, HashSet};
use std::hash::Hash;
use std::iter::FromIterator;
use std::ops::Range;

enum Edge<'a, V: 'a> {
    On(&'a V),
    Off(&'a V),
}

pub struct Sweeper<'a, Idx, V: 'a> {
    indexes: Vec<Idx>,
    active_ranges: HashSet<&'a V>,
    edges: HashMap<Idx, Vec<Edge<'a, V>>>,
    current_index: usize,
}

impl<'a, Idx, V> Iterator for Sweeper<'a, Idx, V>
where
    Idx: Copy + Hash + Ord + PartialEq,
    V: Eq + Hash,
{
    type Item = (Range<Idx>, HashSet<&'a V>);

    fn next(&mut self) -> Option<Self::Item> {
        if self.current_index + 1 < self.indexes.len() {
            let start_index = self.indexes[self.current_index];
            self.current_index += 1;
            let end_index = self.indexes[self.current_index];
            for edge in self.edges.get(&start_index).unwrap() {
                match edge {
                    Edge::On(v) => self.active_ranges.insert(v),
                    Edge::Off(v) => self.active_ranges.remove(v),
                };
            }
            // FIXME: how to get rid of this clone?
            Some((start_index..end_index, self.active_ranges.clone()))
        } else {
            None
        }
    }
}

impl<'a, Idx, V> FromIterator<(Range<Idx>, &'a V)> for Sweeper<'a, Idx, V>
where
    Idx: Copy + Hash + Ord + PartialEq,
    V: Eq + Hash,
{
    fn from_iter<T>(iter: T) -> Self
    where
        T: IntoIterator<Item = (Range<Idx>, &'a V)>,
    {
        let mut builder = SweeperBuilder::new();
        for (range, value) in iter {
            builder.insert(range, value);
        }
        builder.build()
    }
}

pub struct SweeperBuilder<'a, Idx, V: 'a> {
    indexes: Vec<Idx>,
    edges: HashMap<Idx, Vec<Edge<'a, V>>>,
}

impl<'a, Idx, V> SweeperBuilder<'a, Idx, V>
where
    Idx: Copy + Hash + Ord + PartialEq,
    V: Eq + Hash,
{
    pub fn new() -> Self {
        SweeperBuilder {
            indexes: Vec::new(),
            edges: HashMap::new(),
        }
    }

    pub fn insert(&mut self, range: Range<Idx>, value: &'a V) {
        self.indexes.push(range.start);
        self.indexes.push(range.end);
        self.edges
            .entry(range.start)
            .or_default()
            .push(Edge::On(value));
        self.edges
            .entry(range.end)
            .or_default()
            .push(Edge::Off(value));
    }

    pub fn build(mut self) -> Sweeper<'a, Idx, V> {
        self.indexes.sort();
        self.indexes.dedup();
        Sweeper {
            indexes: self.indexes,
            active_ranges: HashSet::new(),
            edges: self.edges,
            current_index: 0,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_simple_range() {
        let ranges = vec![(0..2, "A"), (4..5, "B"), (1..4, "C")];
        let mut builder = SweeperBuilder::new();
        for r in ranges.iter() {
            builder.insert(r.0.clone(), &r.1);
        }
        let mut sweeper = builder.build();
        assert_eq!(sweeper.next(), Some((0..1, vec!["A"].iter().collect())));
        assert_eq!(
            sweeper.next(),
            Some((1..2, vec!["A", "C"].iter().collect()))
        );
        assert_eq!(sweeper.next(), Some((2..4, vec!["C"].iter().collect())));
        assert_eq!(sweeper.next(), Some((4..5, vec!["B"].iter().collect())));
        assert_eq!(sweeper.next(), None);
    }
}
