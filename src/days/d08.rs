use std::collections::BTreeMap;
use std::fs;

#[derive(Debug)]
struct Rig<'a> {
    junction_boxes: Vec<JunctionBox>,
    circuits: Vec<Circuit<'a>>,
    distances: BTreeMap<(usize, usize), i32>,
}

#[derive(Debug)]
struct Circuit<'a> {
    junction_ids: Vec<&'a usize>,
}

#[derive(Debug)]
struct JunctionBox {
    id: usize,
    location: Location,
}

#[derive(Debug)]
struct Location(i32, i32, i32);

impl<'a> Rig<'a> {
    fn new(path: &str) -> Self {
        let raw = fs::read_to_string(path).expect("file must exist");
        let junction_boxes: Vec<JunctionBox> = raw
            .trim()
            .split("\n")
            .enumerate()
            .map(|(id, l)| {
                let mut line_iter = l
                    .split(",")
                    .map(|n| n.parse::<i32>().expect("element must be number"));

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
}

fn distance(x1: &Location, x2: &Location) -> i32 {
    (x2.0 - x1.0).pow(2) + (x2.1 - x1.1).pow(2) + (x2.2 - x1.2).pow(2)
}

#[cfg(test)]
mod d08 {
    use super::*;

    #[test]
    fn test_part1_example() {
        let rig = Rig::new("src/days/inputs/08/example.txt");
        dbg!(rig);
        todo!()
        // assert_eq!(40, manifold.part_1());
    }

    // #[test]
    // fn test_part1() {
    //     let manifold = TachyonManifold::new("src/days/inputs/07/input.txt");
    //     assert_eq!(1570, manifold.part_1());
    // }
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
