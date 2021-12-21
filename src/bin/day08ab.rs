use std::io::{self, Read};

#[derive(Debug)]
struct Node {
    children: Vec<Node>,
    metadata: Vec<usize>,
}

impl Node {
    fn from_iter<T>(iter: &mut T) -> Self
    where
        T: Iterator<Item = usize>,
    {
        let child_count = iter.next().unwrap();
        let metadata_count = iter.next().unwrap();
        let mut children = Vec::new();
        for _ in 0..child_count {
            children.push(Node::from_iter(iter));
        }
        let mut metadata = Vec::new();
        for _ in 0..metadata_count {
            metadata.push(iter.next().unwrap());
        }
        Node { children, metadata }
    }

    fn iter_nodes(&self) -> NodeIter<'_> {
        NodeIter { nodes: vec![self] }
    }

    fn iter_metadata(&self) -> impl Iterator<Item = &usize> {
        self.iter_nodes().flat_map(|n| n.metadata.iter())
    }

    fn value(&self) -> usize {
        if self.children.is_empty() {
            self.metadata.iter().sum()
        } else {
            self.metadata
                .iter()
                .map(|m| {
                    if *m > 0 && *m <= self.children.len() {
                        self.children[*m - 1].value()
                    } else {
                        0
                    }
                })
                .sum()
        }
    }
}

struct NodeIter<'a> {
    nodes: Vec<&'a Node>,
}

impl<'a> Iterator for NodeIter<'a> {
    type Item = &'a Node;

    fn next(&mut self) -> Option<Self::Item> {
        let node_option = self.nodes.pop();
        if let Some(node) = node_option {
            self.nodes.extend(node.children.iter().rev())
        }
        node_option
    }
}

fn main() -> io::Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;
    let mut input_iter = input.split(' ').map(|x| x.parse::<usize>().unwrap());
    let tree = Node::from_iter(&mut input_iter);
    assert_eq!(input_iter.next(), None);

    println!("{}", tree.iter_metadata().sum::<usize>());
    println!("{}", tree.value());

    Ok(())
}
