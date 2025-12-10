use regex::Regex;
use std::fs;

#[derive(Debug)]
pub struct Worksheet {
    exercises: Vec<(Vec<u64>, Operation)>,
}

#[derive(Debug)]
enum Operation {
    Add,
    Multiply,
}

impl Worksheet {
    pub fn new(path: &str) -> Self {
        let raw_worksheet = fs::read_to_string(path).unwrap();

        let re = Regex::new(r"[^\S\n]+").unwrap();
        let single_spaces = re.replace_all(&raw_worksheet, " ");
        let strings: Vec<Vec<&str>> = single_spaces
            .trim()
            .split("\n")
            .map(|l| l.trim().split(" ").collect())
            .collect();

        let mut strings_transposed: Vec<Vec<&str>> = Vec::new();
        for j in 0..strings[0].len() {
            let mut row = Vec::new();
            for i in 0..strings.len() {
                row.push(strings[i][j]);
            }
            strings_transposed.push(row);
        }

        let exercises: Vec<(Vec<u64>, Operation)> = strings_transposed
            .iter()
            .map(|l| {
                let numbers: Vec<u64> = l[0..l.len() - 1]
                    .iter()
                    .map(|n| n.parse().unwrap())
                    .collect();

                let operation: Operation = match *l.last().unwrap() {
                    "+" => Operation::Add,
                    "*" => Operation::Multiply,
                    _ => panic!("Operation not recognized!"),
                };

                (numbers, operation)
            })
            .collect();

        Worksheet { exercises }
    }
    // let reduced: i32 = (1..10).reduce(|acc, e| acc + e).unwrap_or(0);
    pub fn part_1(&self) -> u64 {
        self.exercises
            .iter()
            .map(|e| {
                e.0.iter()
                    .copied()
                    .reduce(|acc, n| match e.1 {
                        Operation::Add => acc + n,
                        Operation::Multiply => acc * n,
                    })
                    .unwrap()
            })
            .sum()
    }

    fn part_2(&self) -> u64 {
        todo!()
    }
}

#[cfg(test)]
mod d06 {
    use super::*;

    #[test]
    fn test_part1_example() {
        let worksheet = Worksheet::new("src/days/inputs/06/example.txt");
        assert_eq!(4277556, worksheet.part_1());
    }

    #[test]
    fn test_part1() {
        let worksheet = Worksheet::new("src/days/inputs/06/input.txt");
        assert_eq!(4693419406682, worksheet.part_1());
    }
}
