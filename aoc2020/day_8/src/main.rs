use std::collections::HashSet;
use std::fs::File;
use std::io::{prelude::*, BufReader};

fn main() {
    let lines = parse_lines(read_lines("input.txt"));
    let mut instruction: i32 = 0;
    let mut accumulator: i32 = 0;
    let mut visited_part_1 = HashSet::new();
    while !visited_part_1.contains(&instruction) {
        visited_part_1.insert(instruction);
        let (action, amount) = &lines[instruction as usize];
        instruction += 1;
        match action.as_ref() {
            "acc" => accumulator += amount,
            "jmp" => instruction += amount - 1,
            _ => (),
        };
    }
    println!("Part 1: {}", accumulator);

    for changed_index in 0..lines.len() {
        if lines[changed_index].0 == "acc" {
            continue;
        }
        instruction = 0;
        accumulator = 0;
        let mut visited_part_2 = HashSet::new();
        while !visited_part_2.contains(&instruction)
            && instruction >= 0
            && instruction as usize <= lines.len()
        {
            if instruction as usize == lines.len() {
                println!("Part 2: {}", accumulator);
                break;
            }
            visited_part_2.insert(instruction);
            let (borrow_action, amount) = &lines[instruction as usize];
            let mut action = borrow_action.clone();
            if instruction as usize == changed_index {
                action = if action == "jmp" {
                    "nop".to_string()
                } else {
                    "jmp".to_string()
                };
            }
            instruction += 1;
            match action.as_ref() {
                "acc" => accumulator += amount,
                "jmp" => instruction += amount - 1,
                _ => (),
            };
        }
    }
}

fn parse_lines(lines: Vec<String>) -> Vec<(String, i32)> {
    lines
        .iter()
        .map(|line| {
            let mut s = line.split(" ");
            (
                s.next().unwrap().to_string(),
                s.next().unwrap().parse::<i32>().unwrap(),
            )
        })
        .collect()
}

fn read_lines(filename: &str) -> Vec<String> {
    let file = File::open(filename).expect("File not found!");
    let reader = BufReader::new(file);
    reader
        .lines()
        .map(|l| l.expect("Can't read line"))
        .collect()
}
