use std::fs;
use std::ops::RangeInclusive;
use std::collections::VecDeque;

#[derive(Debug)]
pub struct Database {
    ranges: Vec<RangeInclusive<u64>>,
    ids: Vec<u64>,
}

impl Database {
    pub fn new(path: &str) -> Self {
        let binding = fs::read_to_string(path)
            .unwrap();

        let (input_ranges, input_ids) = binding
            .trim()
            .split_once("\n\n")
            .unwrap(); 

        let ranges: Vec<RangeInclusive<u64>> = input_ranges
                .split("\n")
                .map(|r| {
                    let (low, high) = r.split_once("-").unwrap();
                    let start: u64 = low.parse().unwrap();
                    let end: u64 = high.parse().unwrap();
                    start..=end
                })
                .collect();

        let ids: Vec<u64> = input_ids
            .split("\n")
            .map(|r| {
                r.parse::<u64>().unwrap()
            })
            .collect();
        
        Database { ranges, ids }
    }

    pub fn part_1(&self) -> usize {
        self
            .ids
            .iter()
            .filter(|id| self.ranges.iter().any(|r| r.contains(id)))
            .count()
    }

    pub fn part_2(&self) -> usize {
        let mut ranges = self.sorted_ranges();
        ranges = self.disjoint_ranges(ranges);
        
        ranges.iter().map(|r| r.clone().count()).sum()
    }

    fn sorted_ranges(&self) -> VecDeque<RangeInclusive<u64>> {
        let mut ranges: Vec<RangeInclusive<u64>> = self.ranges.clone();
        ranges.sort_by_key(|r| *r.start());

        VecDeque::from(ranges)
    }

    fn disjoint_ranges(&self, mut ranges: VecDeque<RangeInclusive<u64>>) -> VecDeque<RangeInclusive<u64>> {
        let mut disjoint_ranges = VecDeque::new();
        let mut start: u64;
        let mut end: u64;

        'outer: while let Some(mut current) = ranges.pop_front() {
            while let Some(next) = ranges.pop_front() {
                if current.end() < next.start() {
                    disjoint_ranges.push_back(current.clone());
                    current = next;
                    continue;
                } else {
                    start = current.start().clone();
                    end = current.end().max(next.end()).clone();
                    ranges.push_front(start..=end);
                    continue 'outer;
                }
            } 
            disjoint_ranges.push_back(current.clone());
        }

        disjoint_ranges
    }
}
    

#[cfg(test)]
mod d05 {
    use super::*;

    #[test]
    fn test_part1_example() {
        let database = Database::new("src/days/inputs/05/example.txt");
        assert_eq!(3, database.part_1());
    }

    #[test]
    fn test_part1() {
        let database = Database::new("src/days/inputs/05/input.txt");
        assert_eq!(664, database.part_1());
    }

    #[test]
    fn test_part2_example() {
        let database = Database::new("src/days/inputs/05/example.txt");
        assert_eq!(14, database.part_2());
    }

    #[test]
    fn test_part2() {
        let database = Database::new("src/days/inputs/05/input.txt");
        assert_eq!(350780324308385, database.part_2());
    }

    #[test]
    fn test_sorted_ranges() {
        let ranges = vec![
            3..=5,
            10..=14,
            16..=20,
            12..=18,
        ];
        let expected = VecDeque::from([
            3..=5,
            10..=14,
            12..=18,
            16..=20,
        ]);

        let database = Database { ranges, ids: Vec::new()};
        assert_eq!(expected, database.sorted_ranges());
    }
    
    #[test]
    fn test_disjoint_ranges() {
        let ranges = vec![
            3..=5,
            10..=14,
            12..=18,
            16..=20,
        ];

        let expected = VecDeque::from([
            3..=5,
            10..=20,
        ]);

        let database = Database { ranges, ids: Vec::new()};
        assert_eq!(expected, database.disjoint_ranges(database.sorted_ranges()));
    }
}
