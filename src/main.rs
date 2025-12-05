use advent_of_code::days::{
    d01::Document
};

fn main() {
    println!("Advent of code 2025!");
    println!("Day 1\n1: {}", Document::new("src/days/inputs/01/p1.txt").part_1());
    println!("2: {}", Document::new("src/days/inputs/01/p1.txt").part_2());
}
