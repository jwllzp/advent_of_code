use advent_of_code::days::{
    d01::Document,
    d02::Products,
    d03::Banks,
    d04::Grid,
    d05::Database,
    d06::{
        Worksheet,
        Worksheet2,
    },
    d07::{
        Graph,
        TachyonManifold,
    },
    d08::Rig,
};

fn main() {
    println!("Advent of code 2025!");
    println!("---------------------------");
    println!(
        "Day 1\n\t1: {}",
        Document::new("src/days/inputs/01/p1.txt").part_1()
    );
    println!(
        "\t2: {}",
        Document::new("src/days/inputs/01/p1.txt").part_2()
    );
    println!("---------------------------");
    println!(
        "Day 2\n\t1: {}",
        Products::new("src/days/inputs/02/p1.txt").part_1()
    );
    println!(
        "\t2: {}",
        Products::new("src/days/inputs/02/p1.txt").part_2()
    );
    println!("---------------------------");
    println!(
        "Day 3\n\t1: {}",
        Banks::new("src/days/inputs/03/input.txt").part_1()
    );
    println!(
        "\t2: {}",
        Banks::new("src/days/inputs/03/input.txt").part_2()
    );
    println!("---------------------------");
    println!(
        "Day 4\n\t1: {}",
        Grid::new("src/days/inputs/04/input.txt").part_1()
    );
    println!(
        "\t2: {}",
        Grid::new("src/days/inputs/04/input.txt").part_2()
    );
    println!("---------------------------");
    println!(
        "Day 5\n\t1: {}",
        Database::new("src/days/inputs/05/input.txt").part_1()
    );
    println!(
        "\t2: {}",
        Database::new("src/days/inputs/05/input.txt").part_2()
    );
    println!("---------------------------");
    println!(
        "Day 6\n\t1: {}",
        Worksheet::new("src/days/inputs/06/input.txt").part_1()
    );
    println!(
        "\t2: {}",
        Worksheet2::new("src/days/inputs/06/input.txt").solve()
    );
    println!("---------------------------");
    println!(
        "Day 7\n\t1: {}",
        TachyonManifold::new("src/days/inputs/07/input.txt").part_1()
    );
    println!(
        "\t2: {}",
        Graph::new("src/days/inputs/07/input.txt").part_2()
    );
    println!("---------------------------");
    println!(
        "Day 8\n\t1: {}",
        Rig::new("src/days/inputs/08/input.txt").part_1(1000)
    );
    println!("\t2: {}", Rig::new("src/days/inputs/08/input.txt").part_2());
}
