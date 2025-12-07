use std::{fs};

pub struct Banks {
    rows: Vec<Bank>,
}

struct Bank {
    batteries: Vec<u64>
}

impl Banks {
    pub fn new(path: &str) -> Self {
        let rows: Vec<Bank> = fs::read_to_string(path)
            .expect("File could not be parsed")
            .trim()
            .split("\n")
            .map(|l| Bank::new(l))
            .collect(); 

        Banks { rows }
    }

    pub fn part_1(&self) -> u64 {
        self.rows.iter().map(|b| b.find_largest_joltage()).sum()
    }

    pub fn part_2(&self) -> u64 {
        self.rows.iter().map(|b| b.find_largest_joltage2(&b.batteries, &12)).sum()
    }
}

impl Bank {
    fn new(line: &str) -> Self {
        let batteries = line.chars().map(|c| c.to_digit(10).unwrap() as u64).collect();
        Bank { batteries }
    }

    fn find_largest_joltage(&self) -> u64 {
        let right = self.batteries.len() - 1;
        
        let mut left_max = (0, 0);
        let mut right_max = (0, 0);

        for i in 0..right {
            if self.batteries[i] > left_max.1 {
                left_max = (i, self.batteries[i]);
                if left_max.1 == 9 { break }
            }
        }

        for i in ((left_max.0+1)..=right).rev() {
            if self.batteries[i] > right_max.1 {
                right_max = (i, self.batteries[i]);
                if right_max.1 == 9 { break }
            }
            
        }
        
        format!("{}{}", left_max.1, right_max.1).parse().unwrap()
    }

    fn find_largest_joltage2(&self, batteries: &[u64], size: &usize) -> u64 {
        let mut max_value = (0, 0);

        for i in 0..(batteries.len() - size + 1) {
            if batteries[i] > max_value.1 {
                max_value = (i, batteries[i]);
                if max_value.1 == 9 { break }
            }
        }
        
        if *size == 1 {
            return max_value.1;
        }

        format!(
            "{}{}",
            max_value.1,
            self.find_largest_joltage2(&batteries[(max_value.0+1)..], &(size-1)),
        ).parse().unwrap()
    }
}

#[cfg(test)]
mod d03 {
    use super::*;
    
    #[test]
    fn part1_case1() {
        let bank = Bank::new("987654321111111");
        assert_eq!(98, bank.find_largest_joltage());
    }

    #[test]
    fn part1_case2() {
        let bank = Bank::new("811111111111119");
        assert_eq!(89, bank.find_largest_joltage());
    }

    #[test]
    fn part1_case3() {
        let bank = Bank::new("234234234234278");
        assert_eq!(78, bank.find_largest_joltage());
    }

    #[test]
    fn part1_case4() {
        let bank = Bank::new("818181911112111");
        assert_eq!(92, bank.find_largest_joltage());
    }
        
    #[test]
    fn part1_example() {
        let banks = Banks::new("src/days/inputs/03/example.txt");
        assert_eq!(357, banks.part_1())
    }

    #[test]
    fn part1() {
        let banks = Banks::new("src/days/inputs/03/input.txt");
        assert_eq!(17408, banks.part_1())
    }

    #[test]
    fn part2_case1() {
        let bank = Bank::new("987654321111111");
        assert_eq!(987654321111, bank.find_largest_joltage2(&bank.batteries, &12));
    }

    #[test]
    fn part2_case2() {
        let bank = Bank::new("811111111111119");
        assert_eq!(811111111119, bank.find_largest_joltage2(&bank.batteries, &12));
    }

    #[test]
    fn part2_case3() {
        let bank = Bank::new("234234234234278");
        assert_eq!(434234234278, bank.find_largest_joltage2(&bank.batteries, &12));
    }

    #[test]
    fn part2_case4() {
        let bank = Bank::new("818181911112111");
        assert_eq!(888911112111, bank.find_largest_joltage2(&bank.batteries, &12));
    }

    #[test]
    fn part2() {
        let banks = Banks::new("src/days/inputs/03/input.txt");
        assert_eq!(172740584266849, banks.part_2())
    }
}
