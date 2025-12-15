use std::collections::{
    BTreeMap,
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
    nodes: BTreeMap<usize, Node>,
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
    Initial(InitialNode),
    Splitter(SplitterNode),
    End(TerminalNode),
}

#[derive(Clone, Debug)]
struct InitialNode {
    id: usize,
    child: usize,
}

#[derive(Clone, Debug)]
struct SplitterNode {
    id: usize,
    children: Vec<usize>,
}

#[derive(Clone, Debug)]
struct TerminalNode {
    id: usize,
    acum_value: usize,
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
        let lines = raw.trim().lines();
        let mut node_id = 0;
        let mut diagram: Vec<Vec<Option<Node>>> = lines
            .map(|l| {
                l.chars()
                    .map(|c| {
                        let node = match c {
                            'S' => {
                                node_id += 1;
                                Some(Node::Initial(InitialNode {
                                    id: node_id,
                                    child: 0,
                                }))
                            }
                            '^' => {
                                node_id += 1;
                                Some(Node::Splitter(SplitterNode {
                                    id: node_id,
                                    children: Vec::new(),
                                }))
                            }
                            _ => None,
                        };
                        node
                    })
                    .collect()
            })
            .collect();

        let mut terminal_nodes = Vec::new();
        for _ in 0..diagram.len() {
            terminal_nodes.push(Some(Node::End(TerminalNode {
                id: node_id,
                acum_value: 0,
            })));
            node_id += 1;
        }
        diagram.push(terminal_nodes);

        let mut nodes: BTreeMap<usize, Node> = BTreeMap::new();
        let initial_node_id: usize = 0;
        let mut terminal_node_ids: Vec<usize> = Vec::new();
        for (row_idx, row) in diagram.iter().enumerate() {
            for (col_idx, col) in row.iter().enumerate() {
                if let Some(node) = col {
                    match node {
                        Node::Initial(initial_node) => {
                            let mut initial_node = initial_node.clone();
                            initial_node.child =
                                Self::get_initial_node_child_ids(
                                    &row_idx, &col_idx, &diagram,
                                );
                            nodes.insert(
                                initial_node.id,
                                Node::Initial(initial_node),
                            );
                        }
                        Node::Splitter(splitter_node) => {
                            let mut splitter_node = splitter_node.clone();
                            splitter_node.children =
                                Self::get_splitter_node_child_ids(
                                    &row_idx, &col_idx, &diagram,
                                );
                            nodes.insert(
                                splitter_node.id,
                                Node::Splitter(splitter_node),
                            );
                        }
                        Node::End(terminal_node) => {
                            nodes.insert(
                                terminal_node.id,
                                Node::End(terminal_node.clone()),
                            );
                            terminal_node_ids.push(terminal_node.id);
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

    fn get_initial_node_child_ids(
        row_idx: &usize,
        col_idx: &usize,
        diagram: &Vec<Vec<Option<Node>>>,
    ) -> usize {
        let mut child_id: usize = 0;

        for i in (row_idx + 1)..diagram.len() {
            if let Some(child) = &diagram[i][*col_idx] {
                match child {
                    Node::Initial(_) => {}
                    Node::Splitter(splitter_node) => {
                        child_id = splitter_node.id;
                    }
                    Node::End(terminal_node) => {
                        child_id = terminal_node.id;
                    }
                }
                break;
            }
        }

        child_id
    }

    fn get_splitter_node_child_ids(
        row_idx: &usize,
        col_idx: &usize,
        diagram: &Vec<Vec<Option<Node>>>,
    ) -> Vec<usize> {
        let mut children: Vec<usize> = Vec::new();

        for i in *row_idx..diagram.len() {
            if let Some(child) = &diagram[i][col_idx - 1] {
                match child {
                    Node::Initial(_) => {}
                    Node::Splitter(splitter_node) => {
                        children.push(splitter_node.id);
                    }
                    Node::End(terminal_node) => {
                        children.push(terminal_node.id);
                    }
                }
                break;
            }
        }

        for i in *row_idx..diagram.len() {
            if let Some(child) = &diagram[i][col_idx + 1] {
                match child {
                    Node::Initial(_) => {}
                    Node::Splitter(splitter_node) => {
                        children.push(splitter_node.id);
                    }
                    Node::End(terminal_node) => {
                        children.push(terminal_node.id);
                    }
                }
                break;
            }
        }

        children
    }

    pub fn propagate(&self, node: &mut Node) {
        match node {
            Node::Initial(initial_node) => {
                self.propagate(
                    self.nodes.get_mut(&initial_node.child).unwrap(),
                );
            }
            Node::Splitter(splitter_node) => {
                splitter_node.children.iter().for_each(|child_id| {
                    self.propagate(self.nodes.get_mut(child_id).unwrap());
                });
            }
            Node::End(terminal_node) => {
                terminal_node.acum_value += 1;
            }
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
        for node in manifold.nodes {
            println!("{:?}", node);
        }
        println!("{:?}", manifold.terminal_node_ids);
        todo!();
        assert_eq!(40, manifold.propagate());
    }

    #[test]
    fn test_part2() {
        todo!();
    }
}
