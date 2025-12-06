use std::{fs};

#[derive(Debug)]
enum Direction {
    Left,
    Right
}

#[derive(Debug)]
struct Rotation {
    direction: Direction,
    steps: i32
}

impl Rotation {
    fn new(direction: Direction, steps: i32) -> Self {
        Rotation { direction, steps }
    }
}

#[derive(Debug)]
pub struct Document {
    rotations: Vec<Rotation>
}

impl Document {
    pub fn new(path: &str) -> Self {
        let file = fs::read_to_string(path).unwrap_or_else(|_| {
            panic!("File not found");});
        
        let rotations: Vec<Rotation> = file
            .trim()
            .split('\n')
            .map(|r| {
                let (direction_str, steps_str) = r.split_at(1);
                let direction = match direction_str {
                    "L" => Direction::Left,
                    "R" => Direction::Right,
                    _ => panic!("Not a direction char")
                };
                let steps = steps_str.parse::<i32>().unwrap();
                Rotation::new(direction, steps)
            })
            .collect();

        Document { rotations }
    }

    pub fn part_1(self) -> usize {
        let cycle_size = 100;
        let mut total_zeros = 0;
        let mut position: i32 = 50;
        let mut new_position: i32;
        for rotation in self.rotations {
            let step = rotation.steps % cycle_size;
            match rotation.direction {
                Direction::Left => {
                    new_position = position - step;
                    if position > step {
                        position = new_position
                    } else if position < step {
                        position = cycle_size + new_position
                    } else {
                        position = 0;
                        total_zeros += 1;
                    }
                },
                Direction::Right => {
                    new_position = position + step;
                    if new_position < cycle_size {
                        position = new_position
                    } else if new_position > cycle_size {
                        position = new_position - cycle_size
                    } else {
                        position = 0;
                        total_zeros += 1;
                    }
                },
            };
        }
        total_zeros
    }
    
    pub fn part_2(self) -> i32 {
        let cycle_size = 100;
        let mut total_zeros = 0;
        let mut position: i32 = 50;
        let mut direction: i32;
        for rotation in self.rotations {
            direction = match rotation.direction {
                Direction::Left => -1,
                Direction::Right => 1,
            };
            for _ in 0..rotation.steps.abs() {
                position = position + direction;
                if position % cycle_size == 0 {
                    total_zeros += 1;
                }
            }
        }
        total_zeros
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn part_1_example() {
        let document = Document::new("src/days/inputs/01/p1_test.txt");
        let answer = document.part_1();
        assert_eq!(3, answer)
    }

    #[test]
    fn part_1() {
        let document = Document::new("src/days/inputs/01/p1.txt");
        let answer = document.part_1();
        assert_eq!(1018, answer)
    }

    #[test]
    fn part_2_example() {
        let document = Document::new("src/days/inputs/01/p1_test.txt");
        let answer = document.part_2();
        assert_eq!(6, answer)
    }

    #[test]
    fn part_2() {
        let document = Document::new("src/days/inputs/01/p1.txt");
        let answer = document.part_2();
        assert_eq!(5815, answer)
    }
}
