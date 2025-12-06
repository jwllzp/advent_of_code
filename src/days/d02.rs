use std::fs;

pub struct Products {
    ranges: Vec<IdRange>,
}

impl Products {
    pub fn new(path: &str) -> Self {
        let raw_ranges = fs::read_to_string(path).unwrap_or_else(|_| panic!("File not found :("));
        let ranges: Vec<IdRange> = raw_ranges
            .trim()
            .split(",")
            .map(|s| {IdRange::new(s)})
            .collect();
        
        Products { ranges }
    }

    pub fn part_1(self) -> u64 {
        self
            .ranges
            .iter()
            .map(|r| r.get_invalid_ids(is_valid_id_part_1).iter().sum::<u64>())
            .sum()
    }

    pub fn part_2(self) -> u64 {
        self
            .ranges
            .iter()
            .map(|r| r.get_invalid_ids(is_valid_id_part_2).iter().sum::<u64>())
            .sum()
    }
}

#[derive(Debug, Clone)]
struct IdRange {
    start: u64,
    end: u64,
}

impl IdRange {
    fn new(range: &str) -> Self {
        let (start_unparsed, end_unparsed) = 
            range
            .split_once("-")
            .expect("range must be in the format xxx-yyy");

        let start: u64 = start_unparsed.parse().unwrap();
        let end: u64 = end_unparsed.parse().unwrap();

        IdRange { start, end }
    }

    fn get_invalid_ids<F>(&self, is_valid_id: F) -> Vec<u64>
    where
        F: Fn(&u64) -> bool,
    {
        (self.start..=self.end)
            .filter(|id| !is_valid_id(id))
            .collect()
    }
}

fn is_valid_id_part_1(id: &u64) -> bool {
    let s: String = id.to_string();
    if s.len() % 2 != 0 {
        return true
    }
    let (left, right) = s.split_at(s.len() / 2);
    left != right
}

fn is_valid_id_part_2(id: &u64) -> bool {
    let s: String = id.to_string();
    for chunk_size in 1..=(s.len() / 2) {
        if s.len() % chunk_size != 0 {
            continue;
        }

        let batches: Vec<String> = s
            .chars()
            .collect::<Vec<char>>()
            .chunks(chunk_size)
            .map(|c| -> String {
                c.iter().collect::<String>()
            })
            .collect(); 

        if batches.windows(2).all(|w| w[0] == w[1]) {
            return false;
        }
    }
    true
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1_example() {
        let document = Products::new("src/days/inputs/02/p1_example.txt");
        let answer = document.part_1();
        assert_eq!(1227775554, answer)
    }

    #[test]
    fn test_part1() {
        let document = Products::new("src/days/inputs/02/p1.txt");
        let answer = document.part_1();
        assert_eq!(44487518055, answer)
    }

    #[test]
    fn test_part2_example() {
        let document = Products::new("src/days/inputs/02/p1_example.txt");
        let answer = document.part_2();
        assert_eq!(4174379265, answer)
    }

    #[test]
    fn test_part2() {
        let document = Products::new("src/days/inputs/02/p1.txt");
        let answer = document.part_2();
        assert_eq!(53481866137, answer)
    }

}
