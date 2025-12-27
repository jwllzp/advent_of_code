use std::collections::BTreeMap;
use std::fs;

#[derive(Debug)]
pub struct Rig<'a> {
    junction_boxes: Vec<JunctionBox>,
    circuits: Vec<Circuit<'a>>,
    distances: BTreeMap<(usize, usize), i64>,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Circuit<'a>(Vec<&'a usize>);

#[derive(Debug)]
struct JunctionBox {
    id: usize,
    location: Location,
}

#[derive(Debug)]
struct Location(i64, i64, i64);

impl<'a> Rig<'a> {
    pub fn new(path: &str) -> Self {
        let raw = fs::read_to_string(path).expect("file must exist");
        let junction_boxes: Vec<JunctionBox> = raw
            .trim()
            .split("\n")
            .enumerate()
            .map(|(id, l)| {
                let mut line_iter = l
                    .split(",")
                    .map(|n| n.parse::<i64>().expect("element must be number"));

                let location = Location(
                    line_iter.next().unwrap(),
                    line_iter.next().unwrap(),
                    line_iter.next().unwrap(),
                );

                JunctionBox { id, location }
            })
            .collect();

        let circuits = Vec::new();
        let mut distances = BTreeMap::new();
        for i in 0..junction_boxes.len() {
            for j in (i + 1)..junction_boxes.len() {
                let distance = distance(
                    &junction_boxes[i].location,
                    &junction_boxes[j].location,
                );
                distances.insert((i, j), distance);
            }
        }

        Rig {
            junction_boxes,
            circuits,
            distances,
        }
    }

    pub fn part_1(&'a mut self, top_n: usize) -> usize {
        let mut sorted_keys: Vec<(&(usize, usize), &i64)> =
            self.distances.iter().map(|(key, val)| (key, val)).collect();

        sorted_keys.sort_by(|a, b| a.1.cmp(b.1));

        for (iteration, (ids, _)) in sorted_keys.iter().enumerate() {
            if iteration > top_n - 1 {
                break;
            }

            let idx1 = self.id_in_circuit(&ids.0);
            let idx2 = self.id_in_circuit(&ids.1);

            match (idx1, idx2) {
                (None, None) => {
                    let circuit = Circuit(vec![&ids.0, &ids.1]);
                    self.circuits.push(circuit);
                }
                (None, Some(idx)) => self.circuits[idx].push(&ids.0),
                (Some(idx), None) => self.circuits[idx].push(&ids.1),
                (Some(r_idx), Some(l_idx)) => {
                    if r_idx == l_idx {
                        continue;
                    }
                    let mut circuit_1;
                    let circuit_2;
                    if r_idx < l_idx {
                        circuit_2 = self.circuits.remove(l_idx).0;
                        circuit_1 = self.circuits.remove(r_idx).0;
                    } else {
                        circuit_1 = self.circuits.remove(r_idx).0;
                        circuit_2 = self.circuits.remove(l_idx).0;
                    }
                    circuit_1.extend(circuit_2);

                    self.circuits.push(Circuit(circuit_1));
                }
            }
        }

        self.circuits.sort_by(|a, b| b.0.len().cmp(&a.0.len()));
        self.circuits.iter().take(3).map(|c| c.0.len()).product()
    }

    fn id_in_circuit(&self, id: &usize) -> Option<usize> {
        self.circuits.iter().position(|c| c.0.contains(&id))
    }
}

impl<'a> Circuit<'a> {
    fn push(&mut self, id: &'a usize) {
        self.0.push(id);
    }
}

fn distance(x1: &Location, x2: &Location) -> i64 {
    let xx = (x2.0 - x1.0).pow(2);
    let yy = (x2.1 - x1.1).pow(2);
    let zz = (x2.2 - x1.2).pow(2);
    xx + yy + zz
}

#[cfg(test)]
mod d08 {
    use super::*;

    #[test]
    fn test_part_1_example() {
        let mut rig = Rig::new("src/days/inputs/08/example.txt");
        assert_eq!(40, rig.part_1(10));
    }

    #[test]
    fn test_part1() {
        let mut rig = Rig::new("src/days/inputs/08/input.txt");
        assert_eq!(121770, rig.part_1(1000));
    }
    //
    // #[test]
    // fn test_part2_example() {
    //     let mut manifold = Graph::new("src/days/inputs/07/example.txt");
    //     assert_eq!(40, manifold.part_2());
    // }
    //
    // #[test]
    // fn test_part2() {
    //     let mut manifold = Graph::new("src/days/inputs/07/input.txt");
    //     assert_eq!(15118009521693, manifold.part_2());
    // }
}
