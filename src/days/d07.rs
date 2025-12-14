use std::collections::{
    HashMap,
    HashSet,
};
use std::fmt;
use std::fs;

#[derive(Debug)]
pub struct TachyonManifold {
    diagram: Vec<Vec<Obstacle>>,
    beams: Vec<Beam>,
}

#[derive(Debug)]
pub struct Graph {
    nodes: HashMap<usize, Node>,
    initial_node_id: usize,
    terminal_node_ids: Vec<usize>,
}

#[derive(Clone, Debug)]
enum Obstacle {
    Space,
    Splitter,
}

#[derive(Clone, Debug)]
enum Node {
    Initial(usize),
    Splitter(SplitterNode),
    End(usize),
}

#[derive(Clone, Debug)]
struct SplitterNode {
    id: usize,
    children: Vec<usize>,
}

#[derive(Debug)]
struct Beam {
    position: usize,
}

impl TachyonManifold {
    pub fn new(path: &str) -> Self {
        let raw = fs::read_to_string(path).unwrap();
        let mut lines = raw.trim().lines();
        let first_line = lines.next().unwrap();
        let beam_start = first_line.find("S").unwrap();

        let diagram: Vec<Vec<Obstacle>> = lines
            .map(|l| {
                l.chars()
                    .map(|c| match c {
                        '.' => Obstacle::Space,
                        '^' => Obstacle::Splitter,
                        _ => panic!(),
                    })
                    .collect()
            })
            .collect();

        let beams = vec![Beam::new(beam_start)];

        TachyonManifold { diagram, beams }
    }

    pub fn part_1(&self) -> usize {
        let diagram_range = 0..self.diagram[0].len();
        let mut splits = 0;
        let mut beam_positions = self.get_beam_postions();
        let mut next_beam_positions: HashSet<usize>;
        let mut left: usize = 0;
        let mut right: usize = 0;

        for line in &self.diagram {
            next_beam_positions = HashSet::new();
            beam_positions.iter().for_each(|b| {
                match line[*b] {
                    Obstacle::Space => {
                        next_beam_positions.insert(*b);
                    }
                    Obstacle::Splitter => {
                        splits += 1;

                        left = b - 1;
                        right = b + 1;

                        if diagram_range.contains(&left) {
                            next_beam_positions.insert(left);
                        }

                        if diagram_range.contains(&right) {
                            next_beam_positions.insert(right);
                        }
                    }
                };
            });
            beam_positions = next_beam_positions;
        }

        splits
    }

    fn get_beam_postions(&self) -> HashSet<usize> {
        self.beams.iter().map(|b| b.position).collect()
    }
}

impl Beam {
    fn new(position: usize) -> Self {
        Beam { position }
    }
}

impl fmt::Display for Obstacle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Obstacle::Space => write!(f, "."),
            Obstacle::Splitter => write!(f, "^"),
        }
    }
}

impl Graph {
    pub fn new(path: &str) -> Self {
        let raw = fs::read_to_string(path).unwrap();
        let mut lines = raw.trim().lines();
        let mut node_id = 0;

        let mut diagram: Vec<Vec<Option<Node>>> = lines
            .map(|l| {
                l.chars()
                    .map(|c| {
                        let node = match c {
                            'S' => Some(Node::Initial(node_id)),
                            '^' => Some(Node::Splitter(SplitterNode {
                                id: node_id,
                                children: Vec::new(),
                            })),
                            _ => None,
                        };
                        node_id += 1;
                        node
                    })
                    .collect()
            })
            .collect();

        let mut terminal_nodes = Vec::new();
        for _ in 0..diagram.len() {
            terminal_nodes.push(Some(Node::End(node_id)));
            node_id += 1;
        }
        diagram.push(terminal_nodes);

        let mut nodes: HashMap<usize, Node> = HashMap::new();
        let initial_node_id: usize = 0;
        let mut terminal_node_ids: Vec<usize> = Vec::new();
        for (row_idx, row) in diagram.iter().enumerate() {
            for (col_idx, col) in row.iter().enumerate() {
                if let Some(node) = col {
                    match node {
                        Node::Initial(id) => {
                            nodes.insert(*id, node.clone());
                        }
                        Node::Splitter(splitter_node) => {
                            let mut splitter_node = splitter_node.clone();
                            splitter_node.children = Self::get_child_ids(
                                &row_idx, &col_idx, &diagram,
                            );
                            nodes.insert(
                                splitter_node.id,
                                Node::Splitter(splitter_node),
                            );
                        }
                        Node::End(id) => {
                            nodes.insert(*id, node.clone());
                            terminal_node_ids.push(*id);
                        }
                    }
                }
            }
        }

        Graph {
            nodes,
            initial_node_id,
            terminal_node_ids,
        }
    }

    fn get_child_ids(
        row_idx: &usize,
        col_idx: &usize,
        diagram: &Vec<Vec<Option<Node>>>,
    ) -> Vec<usize> {
        todo!()
    }

    pub fn propagate(&self) -> usize {
        todo!();
    }
}

impl Node {
    fn new(id: usize, child_ids: Vec<usize>, node_type: NodeType) -> Self {
        Node {
            id,
            child_ids,
            node_type,
        }
    }
}

#[cfg(test)]
mod d07 {
    use super::*;

    #[test]
    fn test_part1_example() {
        let manifold = TachyonManifold::new("src/days/inputs/07/example.txt");
        assert_eq!(21, manifold.part_1());
    }

    #[test]
    fn test_part1() {
        let manifold = TachyonManifold::new("src/days/inputs/07/input.txt");
        assert_eq!(1570, manifold.part_1());
    }

    #[test]
    fn test_part2_example() {
        let manifold = Graph::new("src/days/inputs/07/example.txt");
        assert_eq!(40, manifold.propagate());
    }

    #[test]
    fn test_part2() {
        todo!();
    }
}
