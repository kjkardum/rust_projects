use std::{fs::File, io::prelude::*, io::BufReader};

fn main() {
    let lines = read_input("input.txt");
    let width = lines.iter().next().unwrap().len();
    let mut total = 1;
    let directions = [(1, 1), (1, 3), (1, 5), (1, 7), (2, 1)];
    for i in &directions {
        let trees = lines
            .iter()
            .enumerate()
            .filter(|(index, line)| {
                index % i.0 == 0 && line.chars().nth((index / i.0 * i.1) % width).unwrap() == '#'
            })
            .count();
        total *= trees;
        println!("{:?}: {}", i, trees);
    }
    println!("multiply: {}", total);
}

fn read_input(filename: &str) -> Vec<String> {
    let file = File::open(filename).expect("file not found.");
    let reader = BufReader::new(file);
    reader
        .lines()
        .map(|l| l.expect("Can't read line"))
        .collect()
}
