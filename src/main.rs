use advent_of_code::days::{
    d01::Document,
    d02::Products,
};

fn main() {
    println!("Advent of code 2025!");
    println!("--------------------");
    println!("Day 1\n\t1: {}", Document::new("src/days/inputs/01/p1.txt").part_1());
    println!("\t2: {}", Document::new("src/days/inputs/01/p1.txt").part_2());
    println!("--------------------");
    println!("Day 2\n\t1: {}", Products::new("src/days/inputs/02/p1.txt").part_1());
    println!("\t2: {}", Products::new("src/days/inputs/02/p1.txt").part_2());
}
