use regex::Regex;
use std::fs;

#[derive(Debug)]
pub struct Worksheet {
    exercises: Vec<(Vec<u64>, Operation)>,
}

#[derive(Debug)]
pub struct Worksheet2 {
    exercises: Vec<(u64, Option<Operation>)>,
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
}

impl Worksheet2 {
    pub fn new(path: &str) -> Self {
        let raw_worksheet = fs::read_to_string(path).unwrap();

        let chars: Vec<Vec<char>> = raw_worksheet[0..raw_worksheet.len() - 1]
            .split("\n")
            .map(|l| l.chars().collect())
            .collect();

        let mut chars_transposed: Vec<Vec<char>> = Vec::new();
        for j in 0..chars[0].len() {
            let mut row = Vec::new();
            for i in 0..chars.len() {
                row.push(chars[i][j]);
            }
            chars_transposed.push(row);
        }
        chars_transposed.reverse();

        let exercises: Vec<(u64, Option<Operation>)> = chars_transposed
            .iter()
            .filter(|l| l.iter().any(|c| *c != ' '))
            .map(|l| {
                let number: u64 = l[..l.len() - 1]
                    .iter()
                    .collect::<String>()
                    .trim()
                    .parse()
                    .unwrap();

                let operation: Option<Operation> = match l[l.len() - 1] {
                    '+' => Some(Operation::Add),
                    '*' => Some(Operation::Multiply),
                    _ => None,
                };

                (number, operation)
            })
            .collect();

        Worksheet2 { exercises }
    }

    pub fn solve(&self) -> u64 {
        let mut total: u64 = 0;
        let mut buffer: Vec<u64> = Vec::new();

        for (number, operation) in &self.exercises {
            buffer.push(*number);
            if let Some(o) = operation {
                total += match o {
                    Operation::Add => buffer.iter().sum::<u64>(),
                    Operation::Multiply => buffer.iter().product::<u64>(),
                };
                buffer.clear();
            }
        }

        total
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

    #[test]
    fn test_part2_example() {
        let worksheet = Worksheet2::new("src/days/inputs/06/example.txt");
        println!("{:?}", &worksheet);
        assert_eq!(3263827, worksheet.solve());
    }

    #[test]
    fn test_part2() {
        let worksheet = Worksheet2::new("src/days/inputs/06/input.txt");
        assert_eq!(9029931401920, worksheet.solve());
    }
}
