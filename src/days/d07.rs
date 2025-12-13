use std::collections::HashSet;
use std::fmt;
use std::fs;

#[derive(Debug)]
pub struct TachyonManifold {
    diagram: Vec<Vec<Obstacle>>,
    beams: Vec<Beam>,
}

#[derive(Debug)]
enum Obstacle {
    Space,
    Splitter,
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

    pub fn part_2(&self) -> usize {
        todo!();
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
        let manifold = TachyonManifold::new("src/days/inputs/07/example.txt");
        assert_eq!(40, manifold.part_2());
    }

    #[test]
    fn test_part2() {
        todo!();
    }
}
