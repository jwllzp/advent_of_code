use std::fs;

#[derive(Debug)]
pub struct Grid {
    grid: Vec<Vec<usize>>,
}

impl Grid {
    pub fn new(path: &str) -> Self {
        let lines = fs::read_to_string(path).expect("Can't read lines");

        let rows: Vec<Vec<usize>> = lines
            .trim()
            .split("\n")
            .map(|r| {
                let mut row = Vec::new();
                row.push(0);
                for c in r.chars() {
                    match c {
                        '.' => row.push(0),
                        '@' => row.push(1),
                        _ => panic!("unrecognized character"),
                    }
                }
                row.push(0);
                row
            })
            .collect();

        let padding: Vec<Vec<usize>> = vec![vec![0; rows[0].len()]];
        let grid = [
            padding.as_slice(),
            rows.as_slice(),
            padding.as_slice()
        ].concat();
        
        Grid { grid }
    }

    pub fn part_1(&self) -> usize {
        let max_row = self.grid.len()-1;
        let max_col = self.grid[0].len()-1;
        let mut count = 0;
        let mut count_adjacent;

        for row in 1..max_row {
            for col in 1..max_col {
                if self.grid[row][col] == 0 { 
                    continue;
                }
                count_adjacent = 0;
                count_adjacent += self.grid[row-1][col-1];
                count_adjacent += self.grid[row-1][col];
                count_adjacent += self.grid[row-1][col+1];
                count_adjacent += self.grid[row][col-1];
                count_adjacent += self.grid[row][col+1];
                count_adjacent += self.grid[row+1][col-1];
                count_adjacent += self.grid[row+1][col];
                count_adjacent += self.grid[row+1][col+1];

                if count_adjacent < 4 {
                    count += 1;
                }
            }
        }

        count
    }

    pub fn part_2(&self) -> usize {
        let mut grid = self.grid.clone();
        let max_row = self.grid.len()-1;
        let max_col = self.grid[0].len()-1;
        let mut count = 0;
        let mut count_adjacent;
        let mut remove_indeces: Vec<(usize, usize)> = Vec::new();

        loop {
            for row in 1..max_row {
                for col in 1..max_col {
                    if grid[row][col] == 0 { 
                        continue;
                    }
                    count_adjacent = 0;
                    count_adjacent += grid[row-1][col-1];
                    count_adjacent += grid[row-1][col];
                    count_adjacent += grid[row-1][col+1];
                    count_adjacent += grid[row][col-1];
                    count_adjacent += grid[row][col+1];
                    count_adjacent += grid[row+1][col-1];
                    count_adjacent += grid[row+1][col];
                    count_adjacent += grid[row+1][col+1];

                    if count_adjacent < 4 {
                        count += 1;
                        remove_indeces.push((row, col));
                    }
                }
            }

            if remove_indeces.len() == 0 {
                break;
            }

            while let Some((row, col)) = remove_indeces.pop() {
                grid[row][col] = 0;
            }
        }

        count
    }
}

#[cfg(test)]
mod d04 {
    use super::*;

    #[test]
    fn part1_example() {
        let grid = Grid::new("src/days/inputs/04/example.txt");
        assert_eq!(13, grid.part_1());
    }

    #[test]
    fn part1() {
        let grid = Grid::new("src/days/inputs/04/input.txt");
        assert_eq!(1433, grid.part_1());
    }

    #[test]
    fn part2_example() {
        let grid = Grid::new("src/days/inputs/04/example.txt");
        assert_eq!(43, grid.part_2());
    }

    #[test]
    fn part2() {
        let grid = Grid::new("src/days/inputs/04/input.txt");
        assert_eq!(8616, grid.part_2());
    }
}
